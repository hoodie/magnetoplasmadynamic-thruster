# ⏏️ Magnetoplasmadynamic Thruster 🚀

*a.k.a [MPD](https://crates.io/crates/mpd) meets [Rocket](https://crates.io/crates/rocket).* 

This is a WIP webinterface for [music player daemon](https://www.musicpd.org/).



### T minus 10 .. 9 .. 8 .. 

* [`curl https://sh.rustup.rs -sSf | sh`](https://rustup.rs/) to get rustup
* `git clone https://github.com/hoodie/magnetoplasmadynamic.git`
* `cd magnetoplasmadynamic`

### .. 7 .. 6 .. 5 ..

- `rustup override set nightly`  (because [rocket.rs](https://rocket.rs) only launches at night)
* `export MPD_HOST=localhost:6600 MPD_PW=your_mpd_password`
* **buckle up!**

### .. 4 .. 3 .. 2 ..

* `cargo run`
* **Rocket has launched**
* go to [http://localhost:8000](http://localhost:8000)



### .. 1

* ***"LIFT OFF! WE HAVE A LIFT OFF!"***



##  This is a message to the future:

>  Ground Control to future Hendrik, please implement the following:
>
>  * [ ] error handling
>  * [ ] more freatures ( browsing, adding streams, etc )
>  * [ ] complete REST API
>  * [ ] react.js interface
>  * [ ] take your protein pills
>  * [ ] put your helmet on



Happy Hacking
