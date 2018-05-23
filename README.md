# osAttempt1


`https://os.phil-opp.com/multiboot-kernel/`

``im copying this dude and trying to learn and hopely expand upon this minimal kernel``

## tools ##
* `ld`
* `grub-mkrescue`
* `make`
* `nasm 2.13+`
* `qemu-system-x86_64`
* `xargo`

## to do's ## 

	* clean up make file
	* build rust set up 
	* create rust nightly set up script

## to prepare for rust ##

	* install rustup 
		* curl https://sh.rustup.rs -sSf | sh 
		### [if this error happens:] (https://www.howtodojo.com/2017/09/install-rust-ubuntu-16-04/) ###
		>  error: it looks like you have an existing installation of Rust at:
		>  error: /usr/bin

		*  sudo apt-get -y remove rustc
		*  sudo apt -y autoremove
		*  curl https://sh.rustup.rs -sSf | sh

	* install nightly build 
		* rustup override add nightly

	* install xargo
		* cargo install xargo

## build commands ##

	* nasm `nasm -felf64 $< -o $@`
	* linking `d -n -T $(linker_script) -o $(kernel) $(assembly_object_files)`
	* grub  `grub-mkrescue -o $(iso) build/isofiles` 
	* qemu-run `qemu-system-x86_64 -cdrom $(iso)`

	* building rust os  `RUST_TARGET_PATH=$(pwd) xargo build --target=x86_64-blog_os`
