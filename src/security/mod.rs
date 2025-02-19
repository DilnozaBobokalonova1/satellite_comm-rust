pub mod secure_comm;

/**
 * The point of a security module is to ensure the information is sent and downloaded with the certain
 * set of agreements in mind. For example, if a satellite sends a packet to ground, it has to contain
 * Signature and a set of code to decoding that would only work if the recipient has the access key
 * aka the right to decode the packet. Next, this is also where sat-to-sat communication is withheld
 * within the context of security agreement. No satellite other than the one that collected the initial
 * information is meant to have access to the data that is being relayed. Meaning, if SAT1 uses SAT2 to
 * downlink information to ground, SAT2 cannot decode the information.
 * Hence, we need:
 *
 *      1. Proper encoding
 *      2. Concept and implementation of signature
 *      3. A way for the signature and authentication to work at ground source.
 *      
 */
pub mod encryption;
pub mod key_exchange;
pub mod signature;
