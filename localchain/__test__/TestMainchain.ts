import * as fs from "node:fs";
import {ChildProcess, spawn} from "node:child_process";
import * as path from "node:path";
import * as readline from "node:readline";
import {
    addTeardown,
    cleanHostForDocker,
    getDockerPortMapping,
    getProxy,
    ITeardownable,
} from "./testHelpers";
import process from "node:process";
import child_process from "node:child_process";
import {customAlphabet} from "nanoid";
import * as PortFinder from "portfinder";

const nanoid = customAlphabet('0123456789abcdefghijklmnopqrstuvwxyz', 4);

export default class TestMainchain implements ITeardownable {
    public ip = '127.0.0.1';
    public port: string;
    public loglevel = 'warn';
    #binPath: string;
    #process: ChildProcess;
    #bitcoind: ChildProcess;
    #interfaces: readline.Interface[] = [];
    containerName?: string;
    proxy?: string;

    public get address(): string {
        if (this.proxy) {
            const url = new URL(this.proxy);
            url.searchParams.set('target', `ws://${this.ip}:${this.port}`);
            return url.href;
        }
        return `ws://${this.ip}:${this.port}`;
    }

    constructor(binPath?: string) {
        this.#binPath = binPath ?? `${__dirname}/../../target/debug/ulx-node`;
        this.#binPath = path.resolve(this.#binPath);
        if (!process.env.ULX_USE_DOCKER_BINS && !fs.existsSync(this.#binPath)) {
            throw new Error(`Mainchain binary not found at ${this.#binPath}`);
        }
        addTeardown(this);
    }

    private async startBitcoin(): Promise<string> {
        const rpcPort = await PortFinder.getPortPromise();

        const path = child_process.execSync(`${__dirname}/../../target/debug/ulx-testing-bitcoin`, {encoding: 'utf8'}).trim();

        const tmpDir = fs.mkdtempSync('/tmp/ulx-bitcoin-');

        this.#bitcoind = spawn(path, ['-regtest', '-fallbackfee=0.0001', '-listen=0', `-datadir=${tmpDir}`, '-blockfilterindex', '-txindex', `-rpcport=${rpcPort}`, '-rpcuser=bitcoin', '-rpcpassword=bitcoin'], {
            stdio: ['ignore', 'inherit', 'inherit', "ignore"],
        });

        return cleanHostForDocker(`http://bitcoin:bitcoin@localhost:${rpcPort}`);
    }

    /**
     * Launch and return the localhost url. NOTE: this url will not work cross-docker. You need to use the containerAddress property
     * @param miningThreads
     */
    public async launch(miningThreads = 4): Promise<string> {
        let port = 0;
        let rpcPort = 0;
        let execArgs: string[] = [];
        let containerName: string;
        if (process.env.ULX_USE_DOCKER_BINS) {
            containerName = "miner_" + nanoid();
            this.containerName = containerName;
            this.#binPath = 'docker';
            port = 9944;
            rpcPort = 33344;
            execArgs = ['run', '--rm', `--name=${containerName}`, `-p=0:${port}`, `-p=0:${rpcPort}`, '-e', `RUST_LOG=${this.loglevel},sc_rpc_server=info`,
                'ghcr.io/ulixee/ulixee-miner:dev'];

            if (process.env.ADD_DOCKER_HOST) {
                execArgs.splice(2, 0, `--add-host=host.docker.internal:host-gateway`);
            }
        }

        const bitcoinRpcUrl = await this.startBitcoin();
        execArgs.push('--dev', '--alice', `--miners=${miningThreads}`, `--port=${port}`, `--rpc-port=${rpcPort}`, '--rpc-external', `--bitcoin-rpc-url=${bitcoinRpcUrl}`)
        this.#process = spawn(this.#binPath, execArgs, {
            stdio: ['ignore', 'pipe', 'pipe', "ignore"],
            env: {...process.env, RUST_LOG: `${this.loglevel},sc_rpc_server=info`}
        });

        this.#process.stderr.setEncoding('utf8');
        this.#process.stdout.setEncoding('utf8');
        this.#process.stdout.on('data', (data) => {
            console.log('Main >> %s', data);
        });

        const int1 = readline.createInterface({input: this.#process.stdout}).on('line', line => {
            if (line) console.log('Main >> %s', line);
        });
        this.#interfaces.push(int1);

        this.port = await new Promise<string>((resolve, reject) => {
            this.#process.on('error', (err) => {
                console.warn("Error running mainchain", err);
                reject(err);
            });

            const int2 = readline.createInterface({input: this.#process.stderr}).on('line', line => {
                console.log('Main >> %s', line);
                let match = line.match(/Running JSON-RPC server: addr=([\d.:]+)/);
                if (match) {
                    resolve(match[1].split(':').pop());
                }
            });
            this.#interfaces.push(int2);
        });
        if (this.containerName) {
            this.port = await getDockerPortMapping(this.containerName, 9944);
            this.proxy = cleanHostForDocker(await getProxy());
        }

        console.log(`Ulx Node listening at ${this.address}`);
        return this.address;
    }

    public async teardown(): Promise<void> {
        if (process.env.ULX_USE_DOCKER_BINS) {
            try {
                child_process.execSync(`docker rm -f ${this.containerName}`)
            } catch {
            }
        }
        this.#bitcoind?.kill();
        this.#process?.kill();
        for (const i of this.#interfaces) {
            i.close();
        }
    }
}
