# The Minotaur's Birthday Presents Party

To compile & run (from the `birthday_party` directory)
```
cargo run
```

# Atmospheric Temperature Reading Module

To compile & run (from the `atmospheric_temperature_reading_module` directory)
```
cargo run
```

Reference for setting up Rust: [getting started](https://www.rust-lang.org/learn/get-started)

### Efficiency
The solution randomly generates temperature readings on each thread in O(n) time where `n` is the number of readings taken across all hours.

The solution then incurres a "constant" factor of log(60 * 8) when sorting each hour of readings to find the 5 smallest and 5 largest readings. A quickselect algorithm could have been chosen, but it would complicate the implementation so I decided to use a built-in sort function.

### Correctness
The solution is correct as it generates random temperature readings and sorts them to find the 5 smallest and 5 largest readings for each hour. As shown in the report given as output.

### Progress
The program is implemented in such a way that threads (sensors) wait until all readings have been taken for a given temperature sensor before readings at the next minute are taken. This is to ensure that the readings are taken at the same time across all sensors.

