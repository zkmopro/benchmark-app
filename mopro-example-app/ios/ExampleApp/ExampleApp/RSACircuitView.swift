//
//  MultiplierCircuitView.swift
//  ExampleApp
//
//  Created by User Name on 3/8/24.
//

import JavaScriptCore
import SwiftUI
import moproFFI

struct RSACircuitView: View {
  @State private var textViewText = ""
  @State private var moproWitnessGenTime = ""
  @State private var witnessCalcWitnessGenTime = ""
  @State private var moproProvingTime = ""
  @State private var rapidSnarkProvingTime = ""
  @State private var moproVerificationTime = ""
  @State private var moproProving = false
  @State private var witnessCalcProving = false
  @State private var rapidSnarkProving = false
  @State private var rapidSnarkVerificationTime = ""
  @State private var isProveButtonEnabled = true
  @State private var isVerifyButtonEnabled = false
  @State private var moproWitness: [String] = []
  @State private var generatedProof: Data?
  @State private var publicInputs: Data?
  @State private var witnesscalcWitness: Data?
  @State private var rapidSnarkPublicInputs: UnsafeMutablePointer<UInt8>?
  @State private var rapidSnarkProof: UnsafeMutablePointer<UInt8>?
  @State private var context: JSContext!

  var body: some View {
    NavigationView {
      VStack(spacing: 10) {
        Text("witness generation")
        HStack {
          Button("circom-witness-rs", action: moproFullProve).disabled(moproProving)
          Spacer()
          Text("\(moproWitnessGenTime) ms")
        }.padding()

        HStack {
          Button("Witnesscalc", action: witnesscalc).disabled(witnessCalcProving)
          Spacer()
          Text("\(witnessCalcWitnessGenTime) ms")
        }.padding()

        Text("proof generation")
        HStack {
          Button("ark-works", action: moproFullProve).disabled(moproProving)
          Spacer()
          Text("\(moproProvingTime) ms")
        }.padding()
        HStack {
          Button("Rapidsnark", action: rapidsnarkProve).disabled(rapidSnarkProving)
          Spacer()
          Text("\(rapidSnarkProvingTime) ms")
        }.padding()

        Text("Verification")
        HStack {
          Button("ark-works", action: moproFullProve).disabled(moproProving)
          Spacer()
          Text("\(moproVerificationTime) ms")
        }.padding()

        HStack {
          Button("Rapidsnark", action: rapidsnarkVerify)
          Spacer()
          Text("\(rapidSnarkVerificationTime) ms")
        }.padding()

      }
      .navigationBarTitleDisplayMode(.inline)
      .toolbar {
        ToolbarItem(placement: .principal) {
          VStack {
            Text("RSA Benchmark").font(.headline)
            Text("RSA ベンチマーク").font(.headline)
            Text("Circom Circuit").font(.subheadline)
          }
        }
      }
    }
  }
}

extension RSACircuitView {

  func moproVerify() {
    guard let proof = generatedProof,
      let inputs = publicInputs
    else {
      textViewText += "Proof has not been generated yet.\n"
      return
    }

    Task {
      do {
        let start = CFAbsoluteTimeGetCurrent()

        let isValid = try verifyProof2(proof: proof, publicInput: inputs)
        let end = CFAbsoluteTimeGetCurrent()
        let timeTaken = end - start
        self.moproVerificationTime = String(format: "%.0f", timeTaken * 1000.0)

      } catch let error as MoproError {
        print("\nMoproError: \(error)")
      } catch {
        print("\nUnexpected error: \(error)")
      }
    }
  }

  func moproFullProve() {
    moproProving = true
    Task {
      do {

        // Prepare inputs
        let signature: [String] = [
          "3582320600048169363",
          "7163546589759624213",
          "18262551396327275695",
          "4479772254206047016",
          "1970274621151677644",
          "6547632513799968987",
          "921117808165172908",
          "7155116889028933260",
          "16769940396381196125",
          "17141182191056257954",
          "4376997046052607007",
          "17471823348423771450",
          "16282311012391954891",
          "70286524413490741",
          "1588836847166444745",
          "15693430141227594668",
          "13832254169115286697",
          "15936550641925323613",
          "323842208142565220",
          "6558662646882345749",
          "15268061661646212265",
          "14962976685717212593",
          "15773505053543368901",
          "9586594741348111792",
          "1455720481014374292",
          "13945813312010515080",
          "6352059456732816887",
          "17556873002865047035",
          "2412591065060484384",
          "11512123092407778330",
          "8499281165724578877",
          "12768005853882726493",
        ]

        let modulus: [String] = [
          "13792647154200341559",
          "12773492180790982043",
          "13046321649363433702",
          "10174370803876824128",
          "7282572246071034406",
          "1524365412687682781",
          "4900829043004737418",
          "6195884386932410966",
          "13554217876979843574",
          "17902692039595931737",
          "12433028734895890975",
          "15971442058448435996",
          "4591894758077129763",
          "11258250015882429548",
          "16399550288873254981",
          "8246389845141771315",
          "14040203746442788850",
          "7283856864330834987",
          "12297563098718697441",
          "13560928146585163504",
          "7380926829734048483",
          "14591299561622291080",
          "8439722381984777599",
          "17375431987296514829",
          "16727607878674407272",
          "3233954801381564296",
          "17255435698225160983",
          "15093748890170255670",
          "15810389980847260072",
          "11120056430439037392",
          "5866130971823719482",
          "13327552690270163501",
        ]

        let base_message: [String] = [
          "18114495772705111902", "2254271930739856077",
          "2068851770", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
          "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
        ]

        var inputs = [String: [String]]()
        inputs["signature"] = signature
        inputs["modulus"] = modulus
        inputs["base_message"] = base_message

        let start = CFAbsoluteTimeGetCurrent()

        // Generate Proof
        let res = try fullProve(circuitInputs: inputs)
        print(res)

        let end = CFAbsoluteTimeGetCurrent()
        let timeTaken = end - start
        self.moproWitnessGenTime = String(format: "%.0f", timeTaken * 1000.0)
        self.moproWitnessGenTime = res[0]
        self.moproProvingTime = res[1]
        self.moproVerificationTime = res[2]

        moproProving = false
      } catch {
      }
    }
  }

  func moproProofGen() {
    Task {
      do {
        let start = CFAbsoluteTimeGetCurrent()

        // Generate Proof
        // let proof = try generateProofWithWitness(witness: self.moproWitness)

        let end = CFAbsoluteTimeGetCurrent()
        let timeTaken = end - start
        self.moproProvingTime = String(format: "%.0f", timeTaken * 1000.0)
        //self.generatedProof = proof.proof
        //self.publicInputs = proof.inputs

        isVerifyButtonEnabled = true
      } catch {
      }
    }
  }

  func witnesscalc() {
    witnessCalcProving = true
    do {

      let length = 1024 * 1024 * 1024  // Length of the C string including the null terminator
      _ = UnsafeMutablePointer<CChar>.allocate(capacity: length)
      _ = UnsafeMutablePointer<CChar>.allocate(capacity: length)
      let wtnsSize = UnsafeMutablePointer<UInt>.allocate(capacity: Int(1))
      wtnsSize.initialize(to: UInt(100 * 1024 * 1024))
      let errorSize = UInt(256)
      let wtnsBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: (100 * 1024 * 1024))
      let errorBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(errorSize))

      let datUrl = URL(string: "https://ci-keys.zkmopro.org/cncircuit.dat")
      let inputUrl = URL(string: "https://ci-keys.zkmopro.org/input.json")

      FileDownloader.loadFileSync(url: datUrl!) { (path, error) in
        print("Dat File downloaded to : \(path!)")
      }
      FileDownloader.loadFileSync(url: inputUrl!) { (path, error) in
        print("Input File downloaded to : \(path!)")
      }
        
        if let datPath = Bundle.main.path(forResource: "cncircuit", ofType: "js", inDirectory: "resources") {
          do {
            print("cncircuit")

              guard let datFileHandle = FileHandle(forReadingAtPath: datPath) else {
                print("Failed to open file at path: \(datPath)")
                return
              }
          } catch {
            print("Failed to load bundled snarkjs script: \(error)")
          }
        } else {
            print("else")
        }

      if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
        .first
      {

        let datPath = documentsUrl.appendingPathComponent((datUrl!).lastPathComponent)
        let jsonPath = documentsUrl.appendingPathComponent((inputUrl!).lastPathComponent)

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

        let res = witnesscalc_cncircuit(
          datBuffer, UInt(datFileSize), jsonBuffer, UInt(jsonFileSize), wtnsBuffer, wtnsSize,
          errorBuffer, errorSize)
        self.witnesscalcWitness = Data(bytes: wtnsBuffer, count: Int(wtnsSize.pointee))
        let witness = Data(bytes: wtnsBuffer, count: Int(wtnsSize.pointee))

        let end = CFAbsoluteTimeGetCurrent()
        let timeTaken = end - start
        self.witnessCalcWitnessGenTime = String(format: "%.0f", timeTaken * 1000.0)
        witnessCalcProving = false
      }

    }
  }

  func rapidsnarkProve() {
      print("rapidsnarkProve")
    rapidSnarkProving = true
    let zkeyUrl = URL(string: "https://ci-keys.zkmopro.org/main_final.zkey")
    FileDownloader.loadFileSync(url: zkeyUrl!) { (path, error) in
      print("Zkey File downloaded to : \(path!)")
    }

    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {
      let zkeyPath = documentsUrl.appendingPathComponent((zkeyUrl!).lastPathComponent)
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
      let wtnsSize = self.witnesscalcWitness!.count

      var proofSize: UInt = 4 * 1024 * 1024
      var publicSize: UInt = 4 * 1024 * 1024

      let proofBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(proofSize))
      let publicBuffer = UnsafeMutablePointer<UInt8>.allocate(capacity: Int(publicSize))

      let errorBuffer = UnsafeMutablePointer<Int8>.allocate(capacity: 256)
      let errorMaxSize: UInt = 256

      let start = CFAbsoluteTimeGetCurrent()
      let result = groth16_prover(
        (zkeyFileData as NSData).bytes, UInt(zkeySize),
        (self.witnesscalcWitness as! NSData).bytes, UInt(wtnsSize),
        proofBuffer, &proofSize,
        publicBuffer, &publicSize,
        errorBuffer, errorMaxSize
      )
      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      self.rapidSnarkProvingTime = String(format: "%.0f", timeTaken * 1000.0)
      self.rapidSnarkProof = proofBuffer
      self.rapidSnarkPublicInputs = publicBuffer
      rapidSnarkProving = false
    }
  }

  func rapidsnarkVerify() {
    let vkeyUrl = URL(string: "https://ci-keys.zkmopro.org/main_vkey.json")
    FileDownloader.loadFileSync(url: vkeyUrl!) { (path, error) in
      print("Zkey File downloaded to : \(path!)")
    }

    if let documentsUrl = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
      .first
    {
      let vkeyPath = documentsUrl.appendingPathComponent((vkeyUrl!).lastPathComponent)
      guard let vkeyFileHandle = FileHandle(forReadingAtPath: vkeyPath.path) else {
        print("Failed to open file at path: \(vkeyPath.path)")
        return
      }

      defer {
        vkeyFileHandle.closeFile()
      }

      var vkeyFileData: Data = Data()
      do {
        vkeyFileData = try vkeyFileHandle.readToEnd() ?? Data()
      } catch {
        print("Failed to read file data: \(error)")
      }

      // Get the size of the file
      let vkeyFileSize = vkeyFileData.count

      // Create a buffer
      let vkeyBuffer = vkeyFileData.withUnsafeBytes {
        return $0.baseAddress?.assumingMemoryBound(to: UInt8.self)

      }
      let errorBuffer = UnsafeMutablePointer<Int8>.allocate(capacity: 256)
      let errorMaxSize: UInt = 256

      let start = CFAbsoluteTimeGetCurrent()
      groth16_verify(
        self.rapidSnarkProof, self.rapidSnarkPublicInputs, vkeyBuffer, errorBuffer, errorMaxSize)
      let end = CFAbsoluteTimeGetCurrent()
      let timeTaken = end - start
      self.rapidSnarkVerificationTime = String(format: "%.0f", timeTaken * 1000.0)
    }
  }

  func runSnarkjsWitnessGen() async {
    if let context = JSContext() {
      let swiftVariable = 2 + 3
      context.setObject(
        swiftVariable,
        forKeyedSubscript: "swiftValue" as NSString)

      if let path = Bundle.main.path(forResource: "snarkjs", ofType: "js") {
        do {
          print("call js")

          let script = try String(contentsOfFile: path, encoding: .utf8)
          context.evaluateScript(script)
          // Get the async function from the JSContext
          let fetchDataFunction = context.objectForKeyedSubscript("fetchData")

          // Call the function and handle the promise
          let promise = fetchDataFunction?.call(withArguments: [])
          print("Promise: \(promise)")

          // Handle the promise
          await promise?.invokeMethod(
            "then",
            withArguments: [
              JSValue(
                object: { (result: JSValue) in
                  print("Success: \(result.toString() ?? "No result")")
                }, in: context)
            ])

          // Handle any potential errors
          await promise?.invokeMethod(
            "catch",
            withArguments: [
              JSValue(
                object: { (error: JSValue) in
                  print("Error: \(error.toString() ?? "No error message")")
                }, in: context)
            ])

        } catch {
          print("Failed to load bundled snarkjs script: \(error)")
        }
      }
    }
  }

  func setupJavaScriptContext() {
    if let path = Bundle.main.path(forResource: "snarkjs", ofType: "js") {
      do {
        print("call js")

        let script = try String(contentsOfFile: path, encoding: .utf8)
        context.evaluateScript(script)
      } catch {
        print("Failed to load bundled snarkjs script: \(error)")
      }
    }
  }

  func callJavaScriptAsyncFunction() {
    // Get the async function from the JSContext
    let fetchDataFunction = context.objectForKeyedSubscript("snarkjs")
    let res = context.evaluateScript("snarkjs")
    print(res)

    // Call the function and get the Promise object
    let promise = fetchDataFunction?.call(withArguments: [])

    // Define a JavaScript function to handle the Promise resolution
    let handleResult: @convention(block) (JSValue) -> Void = { result in
      print("Success: \(result.toString() ?? "No result")")
    }

    // Define a JavaScript function to handle the Promise rejection
    let handleError: @convention(block) (JSValue) -> Void = { error in
      print("Error: \(error.toString() ?? "No error message")")
    }

    // Convert the Swift functions to JSValue
    let handleResultJSValue = JSValue(object: handleResult, in: context)
    let handleErrorJSValue = JSValue(object: handleError, in: context)

    // Attach the handlers to the Promise
    promise?.invokeMethod("then", withArguments: [handleResultJSValue])
    promise?.invokeMethod("catch", withArguments: [handleErrorJSValue])
  }

}

//#Preview {
//    CircuitView()
//}
