# ShadowTLS V3+ Protocol Improvements

## Background Issue

ShadowTLS V3 had an inherent design flaw: it added a fixed-length HMAC(PSK) data
(4 bytes) to handshake messages, causing message lengths (such as
ServerFinished) to change from the standard 53 or 69 bytes to 57 or 73 bytes.
This recognizable signature has been successfully identified by detection tools
like Aparecium.

## Improved Solution: Adaptive HMAC Embedding

The new approach uses multiple randomized HMAC embedding strategies instead of
using fixed positions and obvious length modifications:

### Key Improvements

1. **Dynamic Embedding Location**: Instead of adding fixed-length HMAC at the
   end of messages, the embedding position is dynamically determined based on
   the HMAC value itself.

2. **Diversified Embedding Strategies**:
   - Strategy 1: Embed at dynamically calculated positions based on HMAC, using
     XOR operations for subtle modifications
   - Strategy 2: Distribute HMAC embedding across multiple locations in the
     message body with minor modifications
   - Strategy 3: Use variable patterns for distributed modifications while
     maintaining overall byte distribution characteristics

3. **Maintain Standard Lengths**: No longer alters TLS message lengths, keeping
   them completely consistent with standard TLS protocol length characteristics.

4. **Probabilistic Verification**: Client uses corresponding verification
   algorithms that allow for small margins of error, improving protocol
   robustness.

### Technical Advantages

1. **Avoids Fingerprinting**: By not changing message lengths, it avoids obvious
   protocol fingerprints.

2. **Enhanced Concealment**: Through small and unpredictable modifications,
   packet analysis becomes extremely difficult.

3. **Compatibility**: Maintains an appearance completely consistent with
   standard TLS protocol formats.

4. **Analysis Resistance**: The modification method differs with each connection
   and depends on the key and data content, greatly enhancing resistance to
   analysis.

## Security Assessment

The new ShadowTLS V3+ protocol eliminates detectable features while maintaining
the original authentication functionality. Even advanced Deep Packet Inspection
(DPI) systems will struggle to distinguish this traffic from ordinary TLS
traffic.

This improvement has been tested in various network environments and has proven
effective at evading current known detection methods, including specialized
protocol feature detection tools like Aparecium.
