This is an exercise in building a little timezone tool for personal use and edification.


Currently at an early state. Usage:

```
tzwrapper -s timezone1 timezone2
```

Searches the tz database for crude matches on timezone1 and timezone2 (which should be strings with no spaces) and prints the current time.

-s flag saves the results to a config file.

Without timezone arguments, system time and the last result saved with "-s" is printed

Roadmap:
* Add better parsing for the CLI (e.g. the database gives "New_York" which is non-intuitive)
* Curate a subset of the full tz list to reduce errors
* Sort (or add option to sort) by time
* Add options for 12h time and day display since zones may be in different days
* Improve search, maybe with a fuzzy matching algo and dynamic selection from a list
* Add ability to accept and parse arbitrary time inputs for conversion
* Configurable behavior to print a default set of timezones of interest to user - PARTIALLY DONE
* Or, a lookup history that smartly prints most commonly used timezones
* Add a checker that accepts UTC offset and prints a few timezones with that offset.
* Better UI, maybe TUI with an updating clock.






# Rust template
This uses the template from the [Rust Bootcamp](https://s.deza.pe/zjo).
