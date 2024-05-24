# mopro benchmark-app

This is a benchmark app for mopro. It compares different Groth16 adapters in terms of performance for mobile devices.


See [slides](https://docs.google.com/presentation/d/1r4hqV7jPTYf2WjtAzah-w9r5LKbf_-Se9t0HPWCLAs4/edit#slide=id.p) for presentation given during SigSing Residency. Also see [circom benchmarks](https://docs.google.com/spreadsheets/d/1irKg_TOP-yXms8igwCN_3OjVrtFe5gTHkuF0RbrVuho/edit#gid=289866675) for more benchmarks on desktop.

## Running the app

Join the [Testflight](https://testflight.apple.com/join/TBlBDicy).

## Development

Make sure you've followed the general [getting started](https://zkmopro.org/docs/getting-started) steps for mopro and have [mopro-cli](https://github.com/zkmopro/mopro/tree/main/mopro-cli#mopro-cli) installed.

Then run the following commands:

```sh
# Clone the mopro repo
git clone git@github.com:zkmopro/benchmark-app.git

# Go to your newly cloned checkout
cd benchmark-app

# We use the benchmark app Mopro checkout as MOPRO_ROOT
export MOPRO_ROOT=$(PWD)

# Go to mopro-example-app folder
cd mopro-example-app

# Install `mopro` dependencies
mopro deps

# Prepare circuit artifacts
mopro prepare

# Build for iOS
mopro build --platforms ios

# Open in Xcode to run on device
open ios/ExampleApp/ExampleApp.xcworkspace
```

## Future work

1. Expand app with Tachyon support, see https://github.com/zkmopro/mopro/issues/143
2. Add Android support
3. Integrate most promising solutions into mopro, see https://github.com/zkmopro/mopro/issues/146
4. Add more relevant adapters/proof systems as they become available


## Acknowledgements

This app was initially made during [SigSing Residency in Osaka](https://sigsing.vercel.app/), May 2024.