
# LSI

A rust cli tool to get inodes of file(s) similar to `ls -i`

```
ant@teletron2:~/Code/lsi$ target/debug/lsi /
"/run" 2
"/lib64" 16
"/tmp" 14024705
"/boot" 1441793
"/media" 4718593
"/swapfile" 12
"/lib32" 15
"/cdrom" 5767169
"/sbin" 18
"/sys" 1
"/lost+found" 11
"/lib" 14
"/srv" 5373953
"/root" 14942209
"/home" 7471105
"/etc" 14155777
"/usr" 9175041
"/opt" 6815745
"/proc" 1
"/snap" 2359297
"/mnt" 3932161
"/dev" 2
"/libx32" 17
"/var" 2883585
"/bin" 13

==================================================

omicron1-2:lsi ant$ target/debug/lsi
./Cargo.toml 505916
./target 505920
./Cargo.lock 505915
./README.md 505917
./.gitignore 505914
./.git 505831
./src 505918


```

## TODO
 
  * Fix formatting
  * add other flags
  * package for linux / bsd
