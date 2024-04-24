//
//  ContentView.swift
//  LocalchainIOS
//
//  Created by Blake Byrnes on 3/29/24.
//

import BigNumber
import SwiftUI
import UniformTypeIdentifiers

struct PrimaryButton: ButtonStyle {
  func makeBody(configuration: Configuration) -> some View {
    configuration.label
      .padding()
      .background(.accent)
      .frame(maxWidth: .infinity)
      .fontWeight(.bold)
      .foregroundStyle(.white)
      .scaleEffect(configuration.isPressed ? 1.2 : 1)
      .animation(.easeOut(duration: 0.2), value: configuration.isPressed)
  }
}

struct CurvedBackground: Shape {
  func path(in rect: CGRect) -> Path {
    var path = Path()

    // Start at the top left
    path.move(to: CGPoint(x: rect.minX, y: rect.minY))
    // Draw a line to the top right
    path.addLine(to: CGPoint(x: rect.maxX, y: rect.minY))
    // Draw a line to the bottom right
    path.addLine(to: CGPoint(x: rect.maxX, y: rect.maxY))

    // Draw the bottom curve
    path.addCurve(
      to: CGPoint(x: rect.minX, y: rect.maxY), // End at the bottom left
      control1: CGPoint(x: rect.maxX - rect.width * 0.25, y: rect.maxY + rect.height * 0.4), // Control point 1
      control2: CGPoint(x: rect.minX + rect.width * 0.25, y: rect.maxY + rect.height * 0.4) // Control point 2
    )

    // Close the path
    path.closeSubpath()
    return path
  }
}

let lightpurple = Color("lightpurple")
let magenta = Color(UIColor(red: 147.0 / 255.0, green: 33 / 255.0, blue: 166 / 255.0, alpha: 1))
let bgray = Color(#colorLiteral(red: 0.9365568161, green: 0.9451275468, blue: 0.9492741227, alpha: 1))

struct ContentView: View {
  @StateObject var localchainLoader = LocalchainBridge()
  @State var addressText = "Loading your account"
  @State var errorText: String?
  @State var showError = false
  @State var showArgonRequestModalView: Bool = false
  @State var showArgonFileModalView: Bool = false
  @State var argonFileTransfer: ArgonFileTransfer?
  @State var showQrScanner: Bool = false
  @State var toggle = "dashboard"

  let dollarsFormatter = currencyFormatter("$", digits: 0)

  var body: some View {
    NavigationStack {
      VStack(alignment: .center, spacing: 10) {
        ZStack {
          CurvedBackground()
            .stroke(bgray, lineWidth: 0.5)
            .fill(bgray)
            .shadow(radius: 1)

          Picker("Toggle to transactions", selection: $toggle) {
            Text("Dashboard").tag("dashboard")
            Text("Transactions").tag("transactions")
          }
          .pickerStyle(.segmented)
          .padding(.horizontal, 60)
          .padding(.top, 10)
        }.frame(height: 50)
          .padding(.bottom, 30)
          .padding(.top, -20)
          .padding(.horizontal, -30)

        Text("Argon is an inflation-proof stablecoin that uses sound money principles to ensure long-term stability.")
          .font(.caption)
          .foregroundColor(.gray)
          .multilineTextAlignment(.center)
          .padding(.bottom, 30)

        HStack(spacing: 2) {
          Text("\(formatArgons(localchainLoader.balance, digits: 0))")
            .fontWeight(.heavy)
            .font(.system(size: 40.0))
            .foregroundColor(.accentColor)
          Text(
            "\(formatCents(localchainLoader.balance))"
          )
          .bold()
          .font(.system(size: 18.0))
          .foregroundColor(.accentColor)
          .baselineOffset(16.0)
        }

        Button {
          showArgonRequestModalView = true
        } label: {
          Label("Send", systemImage: "paperplane")
            .frame(maxWidth: .infinity)
        }
        .buttonStyle(.borderedProminent)
        .foregroundColor(.white)
        .fontWeight(.bold)
        .padding(.horizontal, 20)

        Button {
          showQrScanner = true
        } label: {
          Label("Accept", systemImage: "qrcode")
            .frame(maxWidth: .infinity)
        }
        .buttonStyle(.bordered)
        .padding(.horizontal, 20)

        Divider()
          .padding(.vertical, 20)

        Text("Current Buying Power")
          .foregroundColor(.accent)
          .fontWeight(.light)
          .padding(.bottom, -10)

        HStack(spacing: 2) {
          Text(
            "\((localchainLoader.currentBuyingPower.toDecimal() / Decimal(1_000.0)).formatted(dollarsFormatter) ?? "Err")"
          )
          .fontWeight(.bold)
          .font(.system(size: 40.0))
          .foregroundColor(.accentColor)
          Text(
            "\(formatCents(localchainLoader.currentBuyingPower))"
          )
          .bold()
          .font(.system(size: 18.0))
          .foregroundColor(.accentColor)
          .baselineOffset(16.0)
        }

        HStack {
          Button {}
            label: { Text("Liquidate")
              .frame(maxWidth: .infinity)
              .foregroundColor(.lightpurple)
              .padding(.horizontal, 10)
              .padding(.vertical, 4)
            }
            .background(
              RoundedRectangle(cornerRadius: 3, style: .continuous)
                .stroke(.lightpurple, lineWidth: 1)
            )

          Button {}
            label: { Text("Learn More")
              .frame(maxWidth: .infinity)
              .foregroundColor(.lightpurple)
              .padding(.horizontal, 10)
              .padding(.vertical, 4)
            }
            .background(
              RoundedRectangle(cornerRadius: 3, style: .continuous)
                .stroke(.lightpurple, lineWidth: 1)
            )
        }
        .padding(.horizontal, 25)

        Divider()
          .padding(.vertical, 20)

        Text("Future Buying Power*")
          .foregroundColor(.accent)
          .fontWeight(.light)
          .padding(.bottom, -10)

        HStack(spacing: 2) {
          Text(
            "\((localchainLoader.futureBuyingPower.toDecimal() / Decimal(1_000.0)).formatted(dollarsFormatter) ?? "Err")"
          )
          .fontWeight(.bold)
          .font(.system(size: 40.0))
          .foregroundColor(.accentColor)
          Text(
            "\(formatCents(localchainLoader.futureBuyingPower))"
          )
          .bold()
          .font(.system(size: 18.0))
          .foregroundColor(.accentColor)
          .baselineOffset(16.0)
        }

        HStack {
          Button {}
            label: { Text("Calculator")
              .frame(maxWidth: .infinity)
              .foregroundColor(.lightpurple)
              .padding(.horizontal, 10)
              .padding(.vertical, 4)
            }
            .background(
              RoundedRectangle(cornerRadius: 3, style: .continuous)
                .stroke(.lightpurple, lineWidth: 1)
            )

          Button {}
            label: { Text("Learn More")
              .frame(maxWidth: .infinity)
              .foregroundColor(.lightpurple)
              .padding(.horizontal, 10)
              .padding(.vertical, 4)
            }
            .background(
              RoundedRectangle(cornerRadius: 3, style: .continuous)
                .stroke(.lightpurple, lineWidth: 1)
            )
        }
        .padding(.horizontal, 25)

        Spacer()
      }
      .toolbar {
        ToolbarItem(placement: .topBarLeading) {
          HStack {
            Image("logo")
              .foregroundColor(magenta)
            Text("Argon by Ulixee")
              .font(.system(size: 18, weight: .bold))
              .foregroundColor(magenta)
          }
        }
        ToolbarItem(placement: .topBarTrailing) {
          Image(systemName: "menu")
        }
      }
      .toolbarBackground(.visible, for: .navigationBar)
      .padding()
    }
    .task {
      do {
        try await localchainLoader.load()

        if let address = localchainLoader.address {
          addressText = "Your Address \(address)"
        }
      } catch let UniffiError.Generic(message) {
        print("Failed to create directory for Localchain \(message)")
        errorText = message
      } catch {
        print("Failed to create directory for Localchain \(error)")
        errorText = "\(error)"
      }
    }
    .sheet(isPresented: $showArgonRequestModalView) {
      ArgonSendSheet(
        isPresented: $showArgonRequestModalView
      )
    }
    .sheet(isPresented: $showArgonFileModalView) {
      ArgonReceivedSheet(
        isPresented: $showArgonFileModalView,
        argonFileTransfer: $argonFileTransfer
      )
      .onDisappear {
        argonFileTransfer = nil
      }
    }
    .sheet(isPresented: $showQrScanner) {
      NavigationView {
        QRScanner(isPresented: $showQrScanner) { message in
          DispatchQueue.main.async {
            showArgonFileModalView = true
            argonFileTransfer = ArgonFileTransfer(name: "", json: message)
          }
        }
        .navigationBarTitle(
          "Load an Argon QR Code",
          displayMode: .inline
        )
        .toolbar {
          Button("", systemImage: "xmark.circle") { showQrScanner = false }
        }
        .toolbarBackground(.visible, for: .navigationBar)
      }
    }
    .onOpenURL { url in
      do {
        let file = try ArgonFileTransfer.fromFile(fileUrl: url)
        argonFileTransfer = file
        showArgonFileModalView = true

      } catch {
        errorText = "Couldn't open the argon file: \(error)"
        showError = true
      }
    }
    .environmentObject(localchainLoader)
    .alert(
      "An error has occurred",
      isPresented: $showError
    ) {}
    message: {
      Text(errorText ?? "Unknown error")
    }
  }
}

#Preview {
  ContentView()
}