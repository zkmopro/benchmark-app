//
//  BenchmarkView.swift
//  ExampleApp
//
//  Created by 鄭雅文 on 2024/5/22.
//

import SwiftUI

struct BenchmarkView: View {
    private var totalFile = 12
    @State private var filesNum = 0
    
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
      Button(action: {
        download()
      }) {
        Text("Download")
      }.disabled(self.filesNum == self.totalFile)
      Text("Files downloaded: \(filesNum) / \(totalFile)")
    
    Button(action: {
      sha256()
      witnessCalcSHA()
      rapidsnarkProveSHA()
    }) {
      Text("SHA256")
    }.disabled(self.filesNum != self.totalFile)
    Text("non-linear constraints: 59281")
    Text("Witness Generation Time").bold()
    Text("circom-witness-rs: \(sha256WitGenTime) ms")
    Text("WitnessCalc: \(sha256WitnessCalcTime) ms")
    Text("Proof Generation Time").bold()
    Text("ark-works: \(sha256ProofGenTime) ms")
    Text("rapidsnark: \(sha256RapidsnarkProveTime) ms")
    //Text("Verification Time").bold()
    //Text("ark-works: \(sha256VerifyTime) ms")
    Button(action: {
      keccak256()
      witnessCalcKeccak()
      rapidsnarkProveKeccak()
    }) {
      Text("Keccak256")
    }.disabled(self.filesNum != self.totalFile)
    Text("non-linear constraints: 150848")
    Text("Witness Generation Time").bold()
    Text("circom-witness-rs: \(keccak256WitGenTime) ms")
    Text("WitnessCalc: \(keccak256WitnessCalcTime) ms")
    Text("Proof Generation Time").bold()
    Text("ark-works: \(keccak256ProofGenTime) ms")
    Text("rapidsnark: \(keccak256RapidsnarkProveTime) ms")
    //Text("Verification Time").bold()
    //Text("ark-works: \(keccak256VerifyTime) ms")
    Button(action: {
      RSA()
      witnessCalcRSA()
      rapidsnarkProveRSA()
    }) {
      Text("RSA")
    }.disabled(self.filesNum != self.totalFile)
    Text("non-linear constraints: 157746")
    Text("Witness Generation Time").bold()
    Text("circom-witness-rs: \(rsaWitGenTime) ms")
    Text("WitnessCalc: \(rsaWitnessCalcTime) ms")
    Text("Proof Generation Time").bold()
    Text("ark-works: \(rsaProofGenTime) ms")
    Text("rapidsnark: \(rsaRapidsnarkProveTime) ms")
    //Text("Verification Time").bold()
    //Text("ark-works: \(rsaVerifyTime) ms")
  }
}

extension BenchmarkView {
    
    func download() {

      FileDownloader.loadFileAsync(url: rsaZkeyUrl!) { (path, error) in
        print("RSA Zkey File downloaded to : \(path!)")
        self.filesNum += 1
      }

      FileDownloader.loadFileAsync(url: rsaGraphrl!) { (path, error) in
        print("RSA Graph File downloaded to : \(path!)")
        self.filesNum += 1
      }
        
        FileDownloader.loadFileAsync(url: rsaDatUrl!) { (path, error) in
          print("RSA Dat File downloaded to : \(path!)")
          self.filesNum += 1
        }

        FileDownloader.loadFileAsync(url: rsaInputrl!) { (path, error) in
          print("RSA Input File downloaded to : \(path!)")
          self.filesNum += 1
        }

      FileDownloader.loadFileAsync(url: keccakZkeyUrl!) { (path, error) in
        print("Keccak Zkey File downloaded to : \(path!)")
        self.filesNum += 1
      }

      FileDownloader.loadFileAsync(url: keccakGraphUrl!) { (path, error) in
        print("Keccak Graph File downloaded to : \(path!)")
        self.filesNum += 1
      }
        
        FileDownloader.loadFileAsync(url: keccakDatUrl!) { (path, error) in
          print("Keccak Dat File downloaded to : \(path!)")
          self.filesNum += 1
        }

        FileDownloader.loadFileAsync(url: keccakInputUrl!) { (path, error) in
          print("Keccak Input File downloaded to : \(path!)")
          self.filesNum += 1
        }

      FileDownloader.loadFileAsync(url: sha256ZkeyUrl!) { (path, error) in
        print("sha Zkey File downloaded to : \(path!)")
        self.filesNum += 1
      }

      FileDownloader.loadFileAsync(url: sha256GraphUrl!) { (path, error) in
        print("sha Graph File downloaded to : \(path!)")
        self.filesNum += 1
      }
        
        FileDownloader.loadFileAsync(url: sha256DatUrl!) { (path, error) in
          print("sha Dat File downloaded to : \(path!)")
          self.filesNum += 1
        }

        FileDownloader.loadFileAsync(url: sha256Inputrl!) { (path, error) in
          print("sha Input File downloaded to : \(path!)")
          self.filesNum += 1
        }
    }

  func sha256() {
    DispatchQueue.main.async {
      if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
        .first
      {
        let zkeyPath = documentsUrl.appendingPathComponent((sha256ZkeyUrl!).lastPathComponent)
        let graphPath = documentsUrl.appendingPathComponent((sha256GraphUrl!).lastPathComponent)
        do {
          let mopro = MoproCircom()
          try mopro.initialize(zkeyPath: zkeyPath.path, graphPath: graphPath.path)
          let inputs = getSHA256Inputs()

          self.sha256WitGenTime = try mopro.generateWitness(circuitInputs: inputs)
          self.sha256ProofGenTime = try mopro.generateProof()
          self.sha256VerifyTime = try mopro.verifyProof()
        } catch {
          print("Error: \(error)")
        }
      }
    }
  }

  func keccak256() {
    DispatchQueue.main.async {
      if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
        .first
      {
        let zkeyPath = documentsUrl.appendingPathComponent((keccakZkeyUrl!).lastPathComponent)
        let graphPath = documentsUrl.appendingPathComponent((keccakGraphUrl!).lastPathComponent)
        do {
          let mopro = MoproCircom()
          try mopro.initialize(zkeyPath: zkeyPath.path, graphPath: graphPath.path)

          let inputs = getKeccakInputs()
          self.keccak256WitGenTime = try mopro.generateWitness(circuitInputs: inputs)
          self.keccak256ProofGenTime = try mopro.generateProof()
          self.keccak256VerifyTime = try mopro.verifyProof()
        } catch {
          print("Error: \(error)")
        }
      }
    }
  }

  func RSA() {
    DispatchQueue.main.async {
      if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
        .first
      {
        let zkeyPath = documentsUrl.appendingPathComponent((rsaZkeyUrl!).lastPathComponent)
        let graphPath = documentsUrl.appendingPathComponent((rsaGraphrl!).lastPathComponent)
        do {
          let mopro = MoproCircom()
          try mopro.initialize(zkeyPath: zkeyPath.path, graphPath: graphPath.path)

          let inputs = getRSAInputs()
          self.rsaWitGenTime = try mopro.generateWitness(circuitInputs: inputs)
          self.rsaProofGenTime = try mopro.generateProof()
          self.rsaVerifyTime = try mopro.verifyProof()
        } catch {
          print("Error: \(error)")
        }
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
      self.keccak256WitnessCalcTime = String(format: "%.0f", timeTaken * 1000.0)
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
      self.sha256WitnessCalcTime = String(format: "%.0f", timeTaken * 1000.0)
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
      self.rsaWitnessCalcTime = String(format: "%.0f", timeTaken * 1000.0)
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
      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      self.keccak256RapidsnarkProveTime = String(format: "%.0f", timeTaken * 1000.0)
      //self.rapidSnarkProof = proofBuffer
      //self.rapidSnarkPublicInputs = publicBuffer
      //rapidSnarkProving = false
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
      self.sha256RapidsnarkProveTime = String(format: "%.0f", timeTaken * 1000.0)
      //self.rapidSnarkProof = proofBuffer
      //self.rapidSnarkPublicInputs = publicBuffer
      //rapidSnarkProving = false
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
      self.rsaRapidsnarkProveTime = String(format: "%.0f", timeTaken * 1000.0)
      //self.rapidSnarkProof = proofBuffer
      //self.rapidSnarkPublicInputs = publicBuffer
      //rapidSnarkProving = false
    }
  }
}
