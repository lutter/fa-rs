This project is a foreign-function interface wrapper of the [Augeas libfa Finite Automata library](http://augeas.net/libfa/index.html) for the [Rust](http://www.rust-lang.org) programming language.

## Installation
This project uses the following software:

 *  [Rust](http://www.rust-lang.org)
 *  [Augeas' libfa Finite Automata library](http://augeas.net/libfa/index.html)
 *  [Cargo](http://crates.io)

You will need the latest dev build of Rust. You can either download a nightly build of the Rust compiler and tools from http://www.rust-lang.org/install.html or clone the GitHub repository, [rust-lang/rust](https://github.com/rust-lang/rust), and [build from source](https://github.com/rust-lang/rust/#building-from-source). On Mac, it is recommended to use [Homebrew](http://brew.sh)'s `rust` formula:

<pre>
# first installation
brew install rust --HEAD

# update
brew reinstall rust --HEAD
</pre>

To install libfa:

 *  Mac with Homebrew:
    <pre>
    brew install augeas
    </pre>

 *  Debian/Ubuntu:
    <pre>
    sudo apt-get install libaugeas-dev
    </pre>

To install Cargo, you will need to build from source. See [Compiling cargo](https://github.com/rust-lang/cargo#compiling-cargo) for instructions. Homebrew users can automate the work of building from source using the `cargo` formula from the https://github.com/dtrebbien/homebrew-misc tap:

<pre>
# first installation
brew tap dtrebbien/misc
brew install cargo --HEAD

# update
brew reinstall cargo --HEAD
</pre>

With the dependencies installed, the `fa` crate is built by running:

<pre>
cargo build
</pre>

To generate the HTML documentation, run:

<pre>
rustdoc --output doc -w html src/fa.rs
</pre>

## License
The wrapper source code is licensed under the [GNU Lesser General Public License](http://www.gnu.org/licenses/lgpl.html), either version 3 of the LGPL, or (at your option) any later version.
