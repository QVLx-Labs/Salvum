# Salvum Security Engine
Official Salvum Repository

-$t@$h -r00r00 -n3wm4n -m0nZSt3r -Matzr3lla -k!r!to

Documentation: https://www.qvlx.com/salvum-1

![image](https://github.com/QVLx-Labs/Salvum/assets/4257899/86d79068-e91c-4820-9e3a-5c2a592efddd)

Integration with QNX Momentics, Wind River Workbench, Lynx Luminosity and other Eclipse-based IDEs

![image](https://github.com/QVLx-Labs/Salvum/assets/4257899/08bc74d0-5664-4601-9de3-fae42a1b6088)

The full range of capabilities:
```
Applets
    |
    +-----------------------------------|-> hw
                                        |-> ls
                                        |-> cat
                                        |-> cp
                                        |-> dump
                                        |-> tree
                                        |-> diff
                                        |-> nm
                                        |-> xxd
                                        |-> ps
                                        |-> rm
                                        |-> find
                                        |-> file
                                        |-> uwc
                                        |-> wc
                                        |-> touch
                                        |-> mkdir
                                        |-> rmdir
                                        |-> sort
                                        |-> ln
                                        |-> readlink
                                        |-> pwd
                                        |-> head
                                        |-> tail
                                        |-> seq
                                        |-> mv
                                        |-> cksum
                                        |-> strip
                                        |-> strings
                                        |-> ustrings
                                        |-> base32
                                        |-> base64
                                        |-> asm
                                        |-> dasm
                                        |-> clc
                                        |-> cpc
                                        |-> rem
                                        |-> relf
                                        |-> relfm
                                        |-> elf
                                        |-> srm
                                        |-> ldd
                                        |-> scrub
                                        |-> wipe
                                        |-> text
                                        |-> ipv4
                                        |-> ipv6
                                        |-> findbytes
                                        |-> merge
                                        |-> deleteall
                                        |-> dedup
                                        |-> ipinfo
                                        |-> ping
                                        |-> infer
                                        |-> binstitch
                                        |-> chardump
                                        |-> bindiff
                                        |-> rmchar
                                        |-> signsig
                                        |-> tabtospc
                                        |-> filestitch
                                        |-> fdtdump
                                        |-> slm_compile
                                        |-> slm_compile_plus
                                        |-> file_converter
                                        |-> tagstr
                                        |-> bzip2
                                        |-> gzip
                                        |-> lz4
                                        |-> lzip
                                        |-> lzop
                                        |-> lzma
                                        |-> xz
                                        |-> zstd
                                        |-> rar
                                        |-> zip
                                        |-> 7zip
                                        |-> Sear
                                        |-> tar
                                        |-> svim
                                        |-> vim
                                        |-> emacs
                                        |-> micro
                                        |-> ascii

Blue Apps
    |
    +-> Basic Tools --------------------|-> CPRNG -------------|-> Chacha
    |                                   |                      |-> Hc128Rng
    |                                   |                      |-> GlassPumpkin
    |                                   |                      |-> nanoid
    |                                   |                      
    |                                   |-> PRNG --------------|-> MersenneTwister
    |                                   |                      |-> rand_hexstr
    |                                   |
    |                                   |-> RNG ---------------|-> OSRand
    |                                   |
    |                                   |-> Entropy -----------|-> ent
    |                                   |                      |-> slm_entropy
    |                                   |                      |-> Tropy
    |                                   |                      |-> NIST SP 800-90B -----|-> spiid
    |                                   |                                               |-> spnoniid
    |                                   |                                               |-> sprestart
    |                                   |
    |                                   |-> Github Keygen
    |                                   |
    |                                   |-> Password Tools ----|-> pass-rs
    |                                   |                      |-> passscore
    |                                   |                      |-> pswd
    |                                   |                      |-> saltyhash
    |                                   |                      |-> zpass
    |                                   |
    |                                   |-> Obfuscators -------|-> ananas
    |                                                          |-> zw
    |
    +-> Code Analyzers -----------------|-> DynamicCode -------|-> Stoke
    |                                   |                      |-> tis-interpreter
    |                                   |                      |-> valgrind
    |                                   |
    |                                   |-> StaticCode --------|-> Advisory Detectors---|-> Cobra
    |                                                          |                        |-> cppcheck
    |                                                          |                        |-> pscan
    |                                                          |
    |                                                          |-> Dependency Checkers--|-> depends
    |                                                          |                        |-> SVF
    |                                                          |
    |                                                          |-> Input Sanitizers-----|-> rust-san
    |                                                          |
    |                                                          |-> Model Checkers-------|-> cbmc
    |                                                          |                        |-> Modex
    |                                                          |                        |-> spin
    |                                                          |
    |                                                          |-> Security Linters-----|-> cpplint 
    |                                                                                   |-> oclint
    |                                                                                   |-> splint
    |
    +-> Cryptography -------------------|-> AES
    |                                   |-> Crypto-detector
    |                                   |-> PGP ---------------|-> pgp decryption
    |                                   |                      |-> pgp encryption
    |                                   |                      |-> pgp key generation
    |                                   |
    |                                   |-> Crypto-URI
    |                                   |-> Cocoon
    |                                   |-> Svanill -----------|-> svanill_encrypt
    |                                   |                      |-> svanill_decrypt
    |                                   |                      |-> svanill_edit
    |                                   |
    |                                   |-> RSA
    |                                   |-> AES_gcm
    |                                   |-> CAST5
    |                                   |-> ChaCha20Poly1305
    |                                   |-> CMAC
    |                                   |-> Triple_DES
    |                                   |-> Blowfish2
    |                                   |-> Threefish
    |                                   |-> Gimli
    |                                   |-> Kuznyechik
    |                                   |-> Magma
    |                                   |-> Rabbit
    |                                   |-> Serpent
    |                                   |-> Sm
    |                                   |-> Speck
    |                                   |-> ECDSA
    |                                   |-> Eax
    |                                   |-> Codecrypt
    |
    +-> Cyclic Redundancy Checkers -----|-> crc16/32/64
    |
    +-> Error Correction Coding --------|-> BCH ---------------|~> Reed_Solomon
    |                                   |
    |                                   |-> Hamming -----------|-> RustyHam
    |                                   |                      |-> SecDed 
    |                                   |
    |                                   |-> LDPC --------------|~> Labrador
    |                                   |
    |                                   |-> Testing -----------|-> FileCorrupter
    |
    +-> Hashers ------------------------|-> Sha256
    |                                   |-> hash
    |                                   |-> rhash
    |                                   |-> hashrat
    |  
    +-> Kernel Hardeners ---------------|-> Buildroot
    |                                   |
    |                                   |-> Yocto -------------|-> SELinux
    |                                   |                      |-> Wind River Linux
    |                                   |
    |                                   |-> Hardening Testing -|-> Linux ---------------|-> checksec
    |                                                                                   |-> KAMain
    | 
    +-> Netloaders --------------------------------------------|-> servF (TFTP, FTP, PXE)
    |                                                          
    +-> Network Analyzers --------------|-> APT Detectors -----|-> cuckoo
    |                                   |
    |                                   |-> DoS Detection -----|-> Gatekeeper
    |                                   |                      |-> Rim
    |                                   |
    |                                   |-> IDS ---------------|-> ARPDefense
    |
    +-> Signers -----------------------------------------------|-> pgp signing
    |                                                          |-> pgp verifying
    |                                                          |-> Watchdog
    |
    +-> Steg Detection ----------------------------------------|-> UnicodeSec
    |
    +-> System Scanners ----------------|-> Auditors ----------|-> armorlib
    |                                   |                      |-> clamav
    |                                   |                      |-> linux malware detect
    |                                   |                      |-> file anomaly finder
    |                                   |                      |-> lynis
    |                                   |                      |-> lemmeknow
    |                                   |                      |-> Yara
    |                                   |
    |                                   |-> Rootkit Detection -|-> chkrootkit
    |
    |-> Vulnerability Databases -------------------------------|-> rCVE
                                                               |-> CVE
                                                               |-> ExploitDB
                                                               |-> bruteforceDB
                                                               |-> FCCID

Red Apps
    |
    +-> Binary Analysis ----------------|-> Runtime -----------|-> avatar2
    |                                   |                      |-> DynamoRIO
    |                                   |                      |-> PySymEmu
    |                                   |                      |-> Qiling
    |                                   |                      |-> usercorn
    |                                   |
    |                                   |-> StaticBin ---------|-> Binary Analysis Platform
    |                                                          |-> Binary Security Check
    |                                                          |-> binwalk
    |                                                          |-> detCVE
    |                                                          |-> chap
    |                                                          |-> Cisco_firmware_extractor
    |                                                          |-> binbloom
    |                                                          |-> elfx86exts
    |                                                          |-> elfutils ------------|-> eu-size
    |                                                                                   |-> eu-compress
    |
    +-> Cracking -----------------------|-> Passwords/Hashes --|-> HashCat
    |                                   |                      |-> JohnTheRipper
    |                                   |                      |-> RainbowCrack --------|-> rcrack
    |                                   |                                               |-> rtgen
    |                                   |                                               |-> rtsort
    |                                   |                                               |-> bruteforce-luks
    |                                   |
    |                                   |-> CRC Reversing -----|-> crchack
    |                                                          |-> reveng
    |
    +-> Decompilers --------------------|-> Boomerang
    |                                   |-> retdec
    |
    +-> Denial of Service --------------|-> Connection killers-|-> TCPkill
    |                                   |
    |                                   |-> Generators --------|-> aSYNcrone
    |                                                          |-> CQHack
    |                                                          |-> t50
    |
    +-> Detection Evasion -------------------------------------|-> fragrouter
    |                                                          |-> pwncat
    |                                                          |-> udptunnel
    |
    +-> Disassemblers ------------------|-> radare2
    |                                   |-> udcli
    |                                   |-> capstone
    |
    +-> Exploit Injection -------------------------------------|-> firmware-mod-kit ----|-> firmextract
    |                                                          |                        |-> firmrebuild
    |                                                          |
    |                                                          |-> Metasploit
    |                                                          |-> Routersploit
    |
    +-> Forensics ----------------------|-> SleuthKit ---------|-> fcat
    |                                   |                      |-> ffind
    |                                   |                      |-> fls
    |                                   |                      |-> fsstat
    |                                   |                      |-> hfind
    |                                   |                      |-> img_stat
    |                                   |                      |-> istat
    |                                   |                      |-> mmls
    |                                   |                      |-> mmstat
    |                                   |                      |-> fiwalk
    |                                   |
    |                                   |-> Stitcher
    |
    +-> Fuzzers ------------------------|-> AFL
    |                                   |-> Honggfuzz
    |
    +-> Man in the Middle --------------|-> Key Databases -----|-> LittleBlackBox
    |                                   |
    |                                   |-> Proxies -----------|-> bettercap
    |                                                          |-> SSH-MITM
    |                                                          |-> SSLSplit
    |
    +-> Raw Tools ----------------------|-> Unpacking ---------|-> vmlinuxtoELF
    |                                   |                      |-> x7z
    |                                   |
    |                                   |-> Decryption --------|-> ciphey
    |                                   |
    |                                   |-> Manipulation ------|-> symtool
    |                                                          |-> ezinject                                    
    |                                                          |-> Keebler
    |                                                          |-> elfbin
    |                                                          |-> dress
    |
    +-> Snoopers -----------------------|-> JTAG --------------|-> JLinkExe
    |                                   |                      |-> openocd
    |                                   |
    |                                   |-> Network Sniffers --|-> dsniff
    |                                   |                      |-> tcpdump
    |                                   |                      |-> TShark
    |                                   |                      |-> sniffglue
    |                                   |
    |                                   |-> Port Scanners -----|-> rang3r
    |                                   |                      |-> sandmap
    |                                   |
    |                                   |-> UART --------------|-> baudrate
    |                                                          |-> miniterm
    |                                                          |-> sbrute
    |
    +-> Spoofers -----------------------|-> ARPSpoof
    |                                   |-> Space_packet_spoof
    |                                   |-> JoesVictimFinder
    |                                   |-> claim-ip
    |                                   |-> FITS_spoof
    |                                   |-> Packit
    |
    |-> Steganography ------------------|-> steg86
    |                                   |-> stegsnow
    |                                   |-> origami-pdf -------|-> pdfcop
    |                                                          |-> pdfdecompress
    |                                                          |-> pdfdecrypt
    |                                                          |-> pdfencrypt
    |                                                          |-> pdfextract
    |                                                          |-> pdfmetadata
    |                                                          |-> pdfattach
    |
    |-> Vendor Tools -------------------|-> QNX ---------------|-> QNX6Extractor
    |                                   |                      |-> dumpIFS
    |                                   |-> VxWorks -----------|-> VxWorks_hash
    |                                   |-> LynxOS ------------|-> LynxFS
    |
    |-> Parsers ------------------------|-> Space_Packet_parse
                                        |-> DHCP_parse
                                        |-> TLS_parse
                                        |-> IPSEC_parse
                                        |-> SSH_parse
                                        |-> FITS_parse
                                        |-> NTP_parse
                                        |-> SNMP_parse
                                        |-> jindex
```
