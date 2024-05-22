
#import "RapidsnarkFramework.h"
#import "WitnesscalcFramework.h"
#import <Foundation/Foundation.h>


BOOL calculateWitnessFromDatAndInputs(NSString *datFilePath, NSDictionary *inputs, NSData **witnessData, NSString **errorString) {
    NSError *error = nil;
    NSData *circuitData = [NSData dataWithContentsOfFile:datFilePath options:0 error:&error];
    if (!circuitData) {
        *errorString = [NSString stringWithFormat:@"Failed to load circuit data: %@", error.localizedDescription];
        return NO;
    }
    
    NSData *jsonData = [NSJSONSerialization dataWithJSONObject:inputs options:0 error:&error];
    if (!jsonData) {
        *errorString = [NSString stringWithFormat:@"Failed to serialize inputs to JSON: %@", error.localizedDescription];
        return NO;
    }
    
    unsigned long wtns_size = 0;
    const unsigned long expectedWitnessSize = 100 * 1024 * 1024;
    NSMutableData *wtnsData = [NSMutableData dataWithLength:expectedWitnessSize]; // Define EXPECTED_WITNESS_SIZE accordingly
    char error_msg[1024] = {0}; // Adjust error message buffer size as needed
    const char *circuit_buffer = (const char *)[circuitData bytes];
    const char *json_buffer = (const char *)[jsonData bytes];
    char *wtns_buffer = (char *)[wtnsData mutableBytes];
    
    wtns_size = [wtnsData length];
    int result = 1;
    
   // int result = witnesscalc_cncircuit(circuit_buffer, [circuitData length], json_buffer, [jsonData length], wtns_buffer, &wtns_size, error_msg, sizeof(error_msg));
    
    if (result != 0) {
        *errorString = [NSString stringWithUTF8String:error_msg];
        return NO;
    }
    
    *witnessData = [wtnsData subdataWithRange:NSMakeRange(0, wtns_size)];
    return YES;
}

