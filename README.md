This is an exercise in building a little timezone tool for personal use and edification.


Current at an early state. Usage:

```
tzwrapper timezone1 timezone 2
```

Searches the tz database for crude matches on timezone1 and timezone2 and prints the current time.

Roadmap:
* Add better parsing for the CLI (e.g. the database gives "New_York" which is non-intuitive)
* Improve search, maybe with a fuzzy matching algo
* Add ability to accept and parse arbitray time inputs for conversion
* Configurable behavior to print a default set of timezones of interest to user
* Or, a lookup history that smartly prints most commonly used timezones
* Add a checker that accepts UTC offset and prints a few timezones with that offset.
* Better UI, maybe TUI with an updating clock.






# Rust template
This uses the template from the [Rust Bootcamp](https://s.deza.pe/zjo).
