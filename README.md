# Blocklists & Allowlists used in my Pi-hole setup

In addition to the links to the blocklists, this repo also contains an opinionated allowlist of domains that are blocked
by some of the blocklists I use, but that I would like to keep unblocked.

I discovered most of the blocklists through [firebog.net](https://firebog.net/).

The little Rust program checks for blocklists that are subsets of other blocklists. It also checks the allowlist for
domains that aren't on any of the blocklists.

Run it with `cargo run --release`.

**Note:** The program currently only parses exact domains.

## Blocklists

### Default blocklists

- https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts

### Suspicious

- https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt
- https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Spam/hosts
- https://v.firebog.net/hosts/static/w3kbl.txt

### Advertising

- https://adaway.org/hosts.txt
- https://v.firebog.net/hosts/AdguardDNS.txt
- https://v.firebog.net/hosts/Admiral.txt
- https://raw.githubusercontent.com/anudeepND/blacklist/master/adservers.txt
- https://v.firebog.net/hosts/Easylist.txt
- https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&showintro=0&mimetype=plaintext
- https://raw.githubusercontent.com/FadeMind/hosts.extras/master/UncheckyAds/hosts
- https://raw.githubusercontent.com/bigdargon/hostsVN/master/hosts

### Tracking & Telemetry

- https://v.firebog.net/hosts/Easyprivacy.txt
- https://v.firebog.net/hosts/Prigent-Ads.txt
- https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.2o7Net/hosts
- https://raw.githubusercontent.com/crazy-max/WindowsSpyBlocker/master/data/hosts/spy.txt
- https://hostfiles.frogeye.fr/firstparty-trackers-hosts.txt

### Malicious

- https://osint.digitalside.it/Threat-Intel/lists/latestdomains.txt
- https://raw.githubusercontent.com/DandelionSprout/adfilt/master/Alternate%20versions%20Anti-Malware%20List/AntiMalwareHosts.txt
- https://v.firebog.net/hosts/Prigent-Crypto.txt
- https://raw.githubusercontent.com/FadeMind/hosts.extras/master/add.Risk/hosts
- https://bitbucket.org/ethanr/dns-blacklists/raw/8575c9f96e5b4a1308f2f12394abd86d0927a4a0/bad_lists/Mandiant_APT1_Report_Appendix_D.txt
- https://phishing.army/download/phishing_army_blocklist_extended.txt
- https://gitlab.com/quidsup/notrack-blocklists/raw/master/notrack-malware.txt
- https://v.firebog.net/hosts/RPiList-Malware.txt
- https://v.firebog.net/hosts/RPiList-Phishing.txt
- https://raw.githubusercontent.com/Spam404/lists/master/main-blacklist.txt
- https://raw.githubusercontent.com/AssoEchap/stalkerware-indicators/master/generated/hosts
- https://urlhaus.abuse.ch/downloads/hostfile/
- https://lists.cyberhost.uk/malware.txt

## Allowlists

- [allowlist](allowlist)
