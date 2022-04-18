# cal-calc

Purpose of this library is to provide a tool for calculating holidays for a given time
period based on a set of rules.

The following rules are supported:

+ Week day: any day of week (e.g. Sunday)
+ Yearly days: same date every year
+ Moveable yearly days: as yearly days, but moved to the next non-weekend day if it falls on a weekday
+ Modified movable yearly days: Move to Friday if it falls on Saturday and to Monday if it falls on Sunday
+ Single day: special holiday valid only in one year
+ Easter offset: day that is calculated relative to Easter sunday
+ Month week day: nth weekday of a given month

## Examples

### uk_settlement_calendar

This examples demonstrates using the library to calculate the UK settlement 
calendar, which is particular interesting because of its complex holiday rules, e.g. 
last Monday of August (Summer bank holiday), fixed day holidays that are moved
to the next working day if the holiday falls on a weekend, or singular holidays
which are not repeated every year, but celebrate unique events like a royal wedding.
