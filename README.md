# ssh_worm

"A computer worm is a standalone malware computer program that replicates itself in order to spread to other computers"

# How it works

In essence, this worm has three main functions (so far):
    1. compile a list of all possible IPs on the local network
    2. test to see if the IPs are online AND if they have port 22 open
        + save the online IPs to a vector
    3. brute force all IPs in said vector.
