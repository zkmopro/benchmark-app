//
//  BenchmarkView.swift
//  ExampleApp
//
//  Created by 鄭雅文 on 2024/5/22.
//

import DeviceKit
import SwiftUI
import SwiftfulLoadingIndicators

struct BenchmarkView: View {
  private var totalFile = 12
  @State private var filesNum = 0
  @State private var isVisible = true
  @State private var runningBenchmark = false

  @State private var sha256WitGenTime = ""
  @State private var sha256ProofGenTime = ""
  @State private var sha256VerifyTime = ""
  @State private var sha256WitnessCalcTime = ""
  @State private var sha256RapidsnarkProveTime = ""

  @State private var keccak256WitGenTime = ""
  @State private var keccak256WitnessCalcTime = ""
  @State private var keccak256ProofGenTime = ""
  @State private var keccak256VerifyTime = ""
  @State private var keccak256RapidsnarkProveTime = ""

  @State private var rsaWitGenTime = ""
  @State private var rsaWitnessCalcTime = ""
  @State private var rsaProofGenTime = ""
  @State private var rsaVerifyTime = ""
  @State private var rsaRapidsnarkProveTime = ""

  @State private var keccak256Witness: Data?
  @State private var sha256Witness: Data?
  @State private var rsaWitness: Data?

  @State private var showWebProverView = false
  @State private var showGoogleFormView = false

  struct WitnessTable: Identifiable {
    let circuit: String
    var witnessRs: String
    var witnessCalc: String
    let id = UUID()

  }

  @State private var witness = [
    WitnessTable(circuit: "keccak256", witnessRs: "0 ms", witnessCalc: "0 ms"),
    WitnessTable(circuit: "sha256", witnessRs: "0 ms", witnessCalc: "0 ms"),
    WitnessTable(circuit: "rsa", witnessRs: "0 ms", witnessCalc: "0 ms"),
  ]

  struct ProofGenTable: Identifiable {
    let circuit: String
    var arkWorks: String
    var rapidSnark: String
    let id = UUID()
  }

  @State private var proofData = [
    ProofGenTable(circuit: "keccak256", arkWorks: "0 ms", rapidSnark: "0 ms"),
    ProofGenTable(circuit: "sha256", arkWorks: "0 ms", rapidSnark: "0 ms"),
    ProofGenTable(circuit: "rsa", arkWorks: "0 ms", rapidSnark: "0 ms"),
  ]

  let rsaZkeyUrl = URL(string: "https://ci-keys.zkmopro.org/rsa_main_final.zkey")
  let rsaGraphrl = URL(string: "https://ci-keys.zkmopro.org/rsa_main.bin")
  let rsaDatUrl = URL(string: "https://ci-keys.zkmopro.org/rsa_main.dat")
  let rsaInputrl = URL(string: "https://ci-keys.zkmopro.org/input.json")
  let keccakZkeyUrl = URL(string: "https://ci-keys.zkmopro.org/keccak256_256_test_final.zkey")
  let keccakGraphUrl = URL(string: "https://ci-keys.zkmopro.org/keccak256_256_test.bin")
  let keccakDatUrl = URL(string: "https://ci-keys.zkmopro.org/keccak256_256_test.dat")
  let keccakInputUrl = URL(string: "https://ci-keys.zkmopro.org/keccak256.json")
  let sha256ZkeyUrl = URL(string: "https://ci-keys.zkmopro.org/sha256_512_final.zkey")
  let sha256GraphUrl = URL(string: "https://ci-keys.zkmopro.org/sha256_512.bin")
  let sha256DatUrl = URL(string: "https://ci-keys.zkmopro.org/sha256_512.dat")
  let sha256Inputrl = URL(string: "https://ci-keys.zkmopro.org/sha256.json")

  var body: some View {
    NavigationStack {
      VStack {
        HStack {
          if self.filesNum != self.totalFile {
            Button(action: {
              download()
            }) {
              Text("Download")
            }.disabled(self.filesNum == self.totalFile).foregroundColor(.yellow)
            Spacer()
            Text("Files downloaded: \(filesNum) / \(totalFile)").foregroundColor(.white)
          }
        }.padding(.horizontal).background(Color(red: 37 / 255, green: 18 / 255, blue: 0 / 255))
          .edgesIgnoringSafeArea(.all)
        HStack {
          if self.runningBenchmark {
            LoadingIndicator(animation: .threeBalls, color: .white).fontWeight(.bold)
              .frame(maxWidth: .infinity)
              .background(Color.yellow)
              .foregroundColor(.white)
              .cornerRadius(10)
              .shadow(color: Color.blue.opacity(0.3), radius: 10, x: 0, y: 5)
              .padding(.horizontal, 10)  // Adds padding on the sides
          } else {
            HStack {
              Button(action: {
                runBenchmark()
              }) {
                Text("Run Benchmark!")
                  .fontWeight(.bold)

              }
              .padding()
              .frame(maxWidth: .infinity)
              .background(self.filesNum != self.totalFile ? Color.gray : Color.yellow)
              .foregroundColor(.white)
              .cornerRadius(10)
              .shadow(color: Color.blue.opacity(0.3), radius: 10, x: 0, y: 5)
              .disabled(self.filesNum != self.totalFile)
              .padding([.leading], 10)
            }
          }
          Button(action: {
            reset()
          }) {
            Text("Reset")
              .fontWeight(.bold)
              .padding()
              .frame(maxWidth: .infinity)
              .background(Color.gray)
              .foregroundColor(.white)
              .cornerRadius(10)
              .shadow(color: Color.gray.opacity(0.3), radius: 10, x: 0, y: 5)
          }
          .padding(.horizontal, 10)  // Adds padding on the sides
        }
        Text("Witness Calculation")
          .fontWeight(.bold)
          .frame(maxWidth: .infinity)
          .foregroundColor(.white)

        List {
          Section(
            header: HStack {
              Text("Circuit")
                .font(.system(size: 14))
                .frame(maxWidth: .infinity, alignment: .leading)
                .font(.headline)
              Text("Witness-rs")
                .font(.system(size: 14))
                .frame(maxWidth: .infinity, alignment: .leading)
                .font(.headline)
              Text("WitnessCalc")
                .font(.system(size: 14))
                .frame(maxWidth: .infinity, alignment: .leading)
                .font(.headline)
            }
          ) {
            ForEach(witness) { wit in
              HStack {
                Text(wit.circuit)
                  .frame(maxWidth: .infinity, alignment: .leading)
                Text(wit.witnessRs)
                  .frame(maxWidth: .infinity, alignment: .leading)
                Text(wit.witnessCalc)
                  .frame(maxWidth: .infinity, alignment: .leading)
              }
            }
          }.listRowBackground(Color.gray)  // Background color for Section 1
        }.listStyle(InsetGroupedListStyle())  // Apply a list style if desired
        Text("Proof Generation")
          .fontWeight(.bold)
          .frame(maxWidth: .infinity)
          .foregroundColor(.white)

        List {
          Section(
            header: HStack {
              Text("Circuit")
                .font(.system(size: 14))
                .frame(maxWidth: .infinity, alignment: .leading)
                .font(.headline)
              Text("ark-works")
                .font(.system(size: 14))
                .frame(maxWidth: .infinity, alignment: .leading)
                .font(.headline)
              Text("rapidsnark")
                .font(.system(size: 14))
                .frame(maxWidth: .infinity, alignment: .leading)
                .font(.headline)
            }
          ) {
            ForEach(proofData) { pf in
              HStack {
                Text(pf.circuit)
                  .frame(maxWidth: .infinity, alignment: .leading)
                Text(pf.arkWorks)
                  .frame(maxWidth: .infinity, alignment: .leading)
                Text(pf.rapidSnark)
                  .frame(maxWidth: .infinity, alignment: .leading)
              }
            }
          }
        }

        HStack {
          Button(action: {
            self.showWebProverView = true
          }) {
            Text("Web Prover")
              .fontWeight(.bold)
              .padding()
              .frame(maxWidth: .infinity)
              .foregroundColor(.orange)
              .cornerRadius(10)
              .shadow(color: Color.orange.opacity(0.3), radius: 10, x: 0, y: 5)

          }.overlay(
            RoundedRectangle(cornerRadius: 10)
              .stroke(Color.orange, lineWidth: 2)  // Border color and width
          ).padding([.leading], 10)  // Adds padding on the sides
          Button(action: {
            copyToClipboard()
            self.showGoogleFormView = true
          }) {
            Text("Submit Results")
              .fontWeight(.bold)
              .padding()
              .frame(maxWidth: .infinity)
              .background(Color.orange)
              .foregroundColor(.white)
              .cornerRadius(10)
              .shadow(color: Color.orange.opacity(0.3), radius: 10, x: 0, y: 5)
              .padding(.horizontal, 10)  // Adds padding on the sides
          }
          //Text("non-linear constraints: 59281")
        }
        Spacer()

        // Text("non-linear constraints: 150848")

        // Text("non-linear constraints: 157746")

      }.background(Color(red: 37 / 255, green: 18 / 255, blue: 0 / 255)).edgesIgnoringSafeArea(
        .all
      )
      .navigationDestination(isPresented: $showWebProverView) {
        WebProverView(url: "https://web-prover.zkmopro.org")
      }
      .navigationDestination(isPresented: $showGoogleFormView) {
        WebProverView(url: "https://forms.gle/gUEzeQmkxsiJAqrF8")
      }
    }
  }
}

extension BenchmarkView {

  func parseData() -> String {
    var res = "\(Device.current),"
    for i in 0...2 {
      res.append(
        " \(self.witness[i].circuit), \(self.witness[i].witnessRs), \(self.witness[i].witnessCalc), \(self.proofData[i].arkWorks), \(self.proofData[i].rapidSnark),"
      )
    }
    return res
  }

  func copyToClipboard() {
    UIPasteboard.general.string = parseData()
  }

  func handleVisibility() {
    self.filesNum += 1
    if self.filesNum == self.totalFile {
      self.isVisible = false
    }
  }

  func download() {

    FileDownloader.loadFileAsync(url: rsaZkeyUrl!) { (path, error) in
      print("RSA Zkey File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: rsaGraphrl!) { (path, error) in
      print("RSA Graph File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: rsaDatUrl!) { (path, error) in
      print("RSA Dat File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: rsaInputrl!) { (path, error) in
      print("RSA Input File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: keccakZkeyUrl!) { (path, error) in
      print("Keccak Zkey File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: keccakGraphUrl!) { (path, error) in
      print("Keccak Graph File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: keccakDatUrl!) { (path, error) in
      print("Keccak Dat File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: keccakInputUrl!) { (path, error) in
      print("Keccak Input File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: sha256ZkeyUrl!) { (path, error) in
      print("sha Zkey File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: sha256GraphUrl!) { (path, error) in
      print("sha Graph File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: sha256DatUrl!) { (path, error) in
      print("sha Dat File downloaded to : \(path!)")
      handleVisibility()
    }

    FileDownloader.loadFileAsync(url: sha256Inputrl!) { (path, error) in
      print("sha Input File downloaded to : \(path!)")
      handleVisibility()
    }
  }

  func reset() {
    for i in 0...2 {
      self.witness[i].witnessRs = "0 ms"
      self.witness[i].witnessCalc = "0 ms"
      self.proofData[i].arkWorks = "0 ms"
      self.proofData[i].rapidSnark = "0 ms"
    }

  }

  func runBenchmark() {
    self.runningBenchmark = true
    DispatchQueue.global(qos: .default).async {
      keccak256()
      witnessCalcKeccak()
      rapidsnarkProveKeccak()
      sha256()
      witnessCalcSHA()
      rapidsnarkProveSHA()
      RSA()
      witnessCalcRSA()
      rapidsnarkProveRSA()
      DispatchQueue.main.async {
        self.runningBenchmark = false
      }
    }

  }

  func sha256() {
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {
      let zkeyPath = documentsUrl.appendingPathComponent((sha256ZkeyUrl!).lastPathComponent)
      let graphPath = documentsUrl.appendingPathComponent((sha256GraphUrl!).lastPathComponent)
      do {
        let mopro = MoproCircom()
        try mopro.initialize(zkeyPath: zkeyPath.path, graphPath: graphPath.path)
        let inputs = getSHA256Inputs()

        let sha256WitGenTime = try mopro.generateWitness(circuitInputs: inputs)
        let sha256ProofGenTime = try mopro.generateProof()
        self.witness[1].witnessRs = String(sha256WitGenTime) + " ms"
        self.proofData[1].arkWorks = String(sha256ProofGenTime) + " ms"
      } catch {
        print("Error: \(error)")
      }
    }

  }

  func keccak256() {
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {
      let zkeyPath = documentsUrl.appendingPathComponent((keccakZkeyUrl!).lastPathComponent)
      let graphPath = documentsUrl.appendingPathComponent((keccakGraphUrl!).lastPathComponent)
      do {
        let mopro = MoproCircom()
        try mopro.initialize(zkeyPath: zkeyPath.path, graphPath: graphPath.path)

        let inputs = getKeccakInputs()
        let keccak256WitGenTime = try mopro.generateWitness(circuitInputs: inputs)
        let keccak256ProofGenTime = try mopro.generateProof()
        self.witness[0].witnessRs = String(keccak256WitGenTime) + " ms"
        self.proofData[0].arkWorks = String(keccak256ProofGenTime) + " ms"
      } catch {
        print("Error: \(error)")
      }
    }

  }

  func RSA() {
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {
      let zkeyPath = documentsUrl.appendingPathComponent((rsaZkeyUrl!).lastPathComponent)
      let graphPath = documentsUrl.appendingPathComponent((rsaGraphrl!).lastPathComponent)
      do {
        let mopro = MoproCircom()
        try mopro.initialize(zkeyPath: zkeyPath.path, graphPath: graphPath.path)

        let inputs = getRSAInputs()
        let rsaWitGenTime = try mopro.generateWitness(circuitInputs: inputs)
        let rsaProofGenTime = try mopro.generateProof()
        self.witness[2].witnessRs = String(rsaWitGenTime) + " ms"
        self.proofData[2].arkWorks = String(rsaProofGenTime) + " ms"
      } catch {
        print("Error: \(error)")
      }
    }
  }

  func witnessCalcKeccak() {
    let wtnsSize = UnsafeMutablePointer<UInt>.allocate(capacity: Int(1))
    wtnsSize.initialize(to: UInt(100 * 1024 * 1024))
    let errorSize = UInt(256)
    let wtnsBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: (100 * 1024 * 1024))
    let errorBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(errorSize))
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {

      let datPath = documentsUrl.appendingPathComponent((keccakDatUrl!).lastPathComponent)
      let jsonPath = documentsUrl.appendingPathComponent((keccakInputUrl!).lastPathComponent)

      guard let datFileHandle = FileHandle(forReadingAtPath: datPath.path) else {
        print("Failed to open file at path: \(datPath.path)")
        return
      }

      defer {
        datFileHandle.closeFile()
      }

      guard let jsonFileHandle = FileHandle(forReadingAtPath: jsonPath.path) else {
        print("Failed to open file at path: \(jsonPath.path)")
        return
      }

      defer {
        jsonFileHandle.closeFile()
      }

      var datFileData: Data = Data()
      do {
        datFileData = try datFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let datFileSize = datFileData.count

      // Create a buffer
      let datBuffer = datFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)
      }

      var jsonFileData: Data = Data()
      do {
        jsonFileData = try jsonFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let jsonFileSize = jsonFileData.count

      // Create a buffer
      let jsonBuffer = jsonFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)
      }

      let start = CFAbsoluteTimeGetCurrent()

      let res = witnesscalc_keccak256_256_test(
        datBuffer, UInt(datFileSize), jsonBuffer, UInt(jsonFileSize), wtnsBuffer, wtnsSize,
        errorBuffer, errorSize)

      self.keccak256Witness = Data(bytes: wtnsBuffer, count: Int(wtnsSize.pointee))
      let witness = Data(bytes: wtnsBuffer, count: Int(wtnsSize.pointee))

      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      let keccak256WitnessCalcTime = String(format: "%.0f", timeTaken * 1000.0)
      self.witness[0].witnessCalc = String(keccak256WitnessCalcTime) + " ms"
    }
  }

  func witnessCalcSHA() {
    let wtnsSize = UnsafeMutablePointer<UInt>.allocate(capacity: Int(1))
    wtnsSize.initialize(to: UInt(100 * 1024 * 1024))
    let errorSize = UInt(256)
    let wtnsBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: (100 * 1024 * 1024))
    let errorBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(errorSize))
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {

      let datPath = documentsUrl.appendingPathComponent((sha256DatUrl!).lastPathComponent)
      let jsonPath = documentsUrl.appendingPathComponent((sha256Inputrl!).lastPathComponent)

      guard let datFileHandle = FileHandle(forReadingAtPath: datPath.path) else {
        print("Failed to open file at path: \(datPath.path)")
        return
      }

      defer {
        datFileHandle.closeFile()
      }

      guard let jsonFileHandle = FileHandle(forReadingAtPath: jsonPath.path) else {
        print("Failed to open file at path: \(jsonPath.path)")
        return
      }

      defer {
        jsonFileHandle.closeFile()
      }

      var datFileData: Data = Data()
      do {
        datFileData = try datFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let datFileSize = datFileData.count

      // Create a buffer
      let datBuffer = datFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)
      }

      var jsonFileData: Data = Data()
      do {
        jsonFileData = try jsonFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let jsonFileSize = jsonFileData.count

      // Create a buffer
      let jsonBuffer = jsonFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)
      }

      let start = CFAbsoluteTimeGetCurrent()

      let res = witnesscalc_sha256_512(
        datBuffer, UInt(datFileSize), jsonBuffer, UInt(jsonFileSize), wtnsBuffer, wtnsSize,
        errorBuffer, errorSize)

      self.sha256Witness = Data(bytes: wtnsBuffer, count: Int(wtnsSize.pointee))
      let witness = Data(bytes: wtnsBuffer, count: Int(wtnsSize.pointee))

      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      let sha256WitnessCalcTime = String(format: "%.0f", timeTaken * 1000.0)
      self.witness[1].witnessCalc = String(sha256WitnessCalcTime) + " ms"
    }
  }

  func witnessCalcRSA() {
    let wtnsSize = UnsafeMutablePointer<UInt>.allocate(capacity: Int(1))
    wtnsSize.initialize(to: UInt(100 * 1024 * 1024))
    let errorSize = UInt(256)
    let wtnsBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: (100 * 1024 * 1024))
    let errorBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(errorSize))
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {

      let datPath = documentsUrl.appendingPathComponent((rsaDatUrl!).lastPathComponent)
      let jsonPath = documentsUrl.appendingPathComponent((rsaInputrl!).lastPathComponent)

      guard let datFileHandle = FileHandle(forReadingAtPath: datPath.path) else {
        print("Failed to open file at path: \(datPath.path)")
        return
      }

      defer {
        datFileHandle.closeFile()
      }

      guard let jsonFileHandle = FileHandle(forReadingAtPath: jsonPath.path) else {
        print("Failed to open file at path: \(jsonPath.path)")
        return
      }

      defer {
        jsonFileHandle.closeFile()
      }

      var datFileData: Data = Data()
      do {
        datFileData = try datFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let datFileSize = datFileData.count

      // Create a buffer
      let datBuffer = datFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)
      }

      var jsonFileData: Data = Data()
      do {
        jsonFileData = try jsonFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let jsonFileSize = jsonFileData.count

      // Create a buffer
      let jsonBuffer = jsonFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)
      }

      let start = CFAbsoluteTimeGetCurrent()

      let res = witnesscalc_rsa_main(
        datBuffer, UInt(datFileSize), jsonBuffer, UInt(jsonFileSize), wtnsBuffer, wtnsSize,
        errorBuffer, errorSize)

      self.rsaWitness = Data(bytes: wtnsBuffer, count: Int(wtnsSize.pointee))
      let witness = Data(bytes: wtnsBuffer, count: Int(wtnsSize.pointee))

      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      let rsaWitnessCalcTime = String(format: "%.0f", timeTaken * 1000.0)
      self.witness[2].witnessCalc = String(rsaWitnessCalcTime) + " ms"
    }
  }

  struct Proof: Codable {
    let piA: [String]
    let piB: [[String]]
    let piC: [String]
    let proofProtocol: String

    enum CodingKeys: String, CodingKey {
      case piA = "pi_a"
      case piB = "pi_b"
      case piC = "pi_c"
      case proofProtocol = "protocol"
    }
  }

  func rapidsnarkProveKeccak() {
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {
      let zkeyPath = documentsUrl.appendingPathComponent((keccakZkeyUrl!).lastPathComponent)
      guard let zkeyFileHandle = FileHandle(forReadingAtPath: zkeyPath.path) else {
        print("Failed to open file at path: \(zkeyPath.path)")
        return
      }

      defer {
        zkeyFileHandle.closeFile()
      }

      var zkeyFileData: Data = Data()
      do {
        zkeyFileData = try zkeyFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let zkeyFileSize = zkeyFileData.count

      // Create a buffer
      let zkeyBuffer = zkeyFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)

      }

      let zkeySize = zkeyFileData.count
      let wtnsSize = self.keccak256Witness!.count

      var proofSize: UInt = 4 * 1024 * 1024
      var publicSize: UInt = 4 * 1024 * 1024

      let proofBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(proofSize))
      let publicBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(publicSize))

      let errorBuffer = UnsafeMutablePointer<Int8>.allocate(capacity: 256)
      let errorMaxSize: UInt = 256

      let start = CFAbsoluteTimeGetCurrent()
      let result = groth16_prover(
        (zkeyFileData as NSData).bytes, UInt(zkeySize),
        (self.keccak256Witness as! NSData).bytes, UInt(wtnsSize),
        proofBuffer, &proofSize,
        publicBuffer, &publicSize,
        errorBuffer, errorMaxSize
      )
      var proofData = Data(bytes: proofBuffer, count: Int(proofSize))
      print(proofData)
      let proofNullIndex = proofData.firstIndex(of: 0x00)!
      proofData = proofData[0..<proofNullIndex]
      do {
        let proof = try JSONDecoder().decode(Proof.self, from: proofData)
        print(proof)
      } catch {
        print("error")
      }

      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      let keccak256RapidsnarkProveTime = String(format: "%.0f", timeTaken * 1000.0)
      self.proofData[0].rapidSnark = String(keccak256RapidsnarkProveTime) + " ms"
    }
  }

  func rapidsnarkProveSHA() {
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {
      let zkeyPath = documentsUrl.appendingPathComponent((sha256ZkeyUrl!).lastPathComponent)
      guard let zkeyFileHandle = FileHandle(forReadingAtPath: zkeyPath.path) else {
        print("Failed to open file at path: \(zkeyPath.path)")
        return
      }

      defer {
        zkeyFileHandle.closeFile()
      }

      var zkeyFileData: Data = Data()
      do {
        zkeyFileData = try zkeyFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let zkeyFileSize = zkeyFileData.count

      // Create a buffer
      let zkeyBuffer = zkeyFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)

      }

      let zkeySize = zkeyFileData.count
      let wtnsSize = self.sha256Witness!.count

      var proofSize: UInt = 4 * 1024 * 1024
      var publicSize: UInt = 4 * 1024 * 1024

      let proofBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(proofSize))
      let publicBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(publicSize))

      let errorBuffer = UnsafeMutablePointer<Int8>.allocate(capacity: 256)
      let errorMaxSize: UInt = 256

      let start = CFAbsoluteTimeGetCurrent()
      let result = groth16_prover(
        (zkeyFileData as NSData).bytes, UInt(zkeySize),
        (self.sha256Witness as! NSData).bytes, UInt(wtnsSize),
        proofBuffer, &proofSize,
        publicBuffer, &publicSize,
        errorBuffer, errorMaxSize
      )
      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      let sha256RapidsnarkProveTime = String(format: "%.0f", timeTaken * 1000.0)
      self.proofData[1].rapidSnark = String(sha256RapidsnarkProveTime) + " ms"
    }
  }

  func rapidsnarkProveRSA() {
    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {
      let zkeyPath = documentsUrl.appendingPathComponent((rsaZkeyUrl!).lastPathComponent)
      guard let zkeyFileHandle = FileHandle(forReadingAtPath: zkeyPath.path) else {
        print("Failed to open file at path: \(zkeyPath.path)")
        return
      }

      defer {
        zkeyFileHandle.closeFile()
      }

      var zkeyFileData: Data = Data()
      do {
        zkeyFileData = try zkeyFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let zkeyFileSize = zkeyFileData.count

      // Create a buffer
      let zkeyBuffer = zkeyFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)

      }

      let zkeySize = zkeyFileData.count
      let wtnsSize = self.rsaWitness!.count

      var proofSize: UInt = 4 * 1024 * 1024
      var publicSize: UInt = 4 * 1024 * 1024

      let proofBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(proofSize))
      let publicBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(publicSize))

      let errorBuffer = UnsafeMutablePointer<Int8>.allocate(capacity: 256)
      let errorMaxSize: UInt = 256

      let start = CFAbsoluteTimeGetCurrent()
      let result = groth16_prover(
        (zkeyFileData as NSData).bytes, UInt(zkeySize),
        (self.rsaWitness as! NSData).bytes, UInt(wtnsSize),
        proofBuffer, &proofSize,
        publicBuffer, &publicSize,
        errorBuffer, errorMaxSize
      )
      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      let rsaRapidsnarkProveTime = String(format: "%.0f", timeTaken * 1000.0)
      self.proofData[2].rapidSnark = String(rsaRapidsnarkProveTime) + " ms"
    }
  }
}
