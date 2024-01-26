import * as fs from "node:fs";
import {ChildProcess, spawn} from "node:child_process";
import * as path from "node:path";
import * as readline from "node:readline";
import {addTeardown, ITeardownable} from "./testHelpers";

export default class TestMainchain implements ITeardownable {
    public address: string;
    #binPath: string;
    #process: ChildProcess;
    #interfaces: readline.Interface[] = [];


    constructor(binPath?: string) {
        this.#binPath = binPath ?? `${__dirname}/../../target/release/ulx-node`;
        if (!fs.existsSync(this.#binPath)) {
            throw new Error(`Mainchain binary not found at ${this.#binPath}`);
        }
        addTeardown(this);
    }

    public async launch(miningThreads = 4): Promise<string> {
        console.log('launching ulx-node from', path.dirname(path.resolve(this.#binPath)))
        this.#process = spawn(path.resolve(this.#binPath), ['--dev', '--alice', `--miners=${miningThreads}`, '--port=0'], {
            stdio: ['ignore', 'pipe', 'pipe', "ignore"],
            // NOTE: if you go lower than info, we can't figure out the port
            env: {...process.env, RUST_LOG: 'info'}
        });

        this.#process.stderr.setEncoding('utf8');
        this.#process.stdout.setEncoding('utf8');
        this.#process.stdout.on('data', (data) => {
            console.log('Main >> %s', data);
        });

        const i = readline.createInterface({input: this.#process.stdout}).on('line', line => {
            if (line) console.log('Main >> %s', line);
        });
        this.#interfaces.push(i);

        this.address = await new Promise<string>((resolve, reject) => {
            this.#process.on('error', (err) => {
                console.warn("Error running mainchain", err);
                reject(err);
            });

            const i = readline.createInterface({input: this.#process.stderr}).on('line', line => {
                console.log('Main >> %s', line);
                let match = line.match(/Running JSON-RPC server: addr=127.0.0.1:(\d+)/);
                if (match) {
                    resolve(`ws://127.0.0.1:${match[1]}`);
                }
            });
            this.#interfaces.push(i);
        });
        return this.address;
    }

    public async teardown(): Promise<void> {
        this.#process?.kill();
        for (const i of this.#interfaces) {
            i.close();
        }
    }
}