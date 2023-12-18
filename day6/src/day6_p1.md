#### Using records (input) as reference

For each distance record stablished in a specific time, you need to calculate
all possible ways to beat the record.

#### Constraints

To make the boat move you need to press a button to set the boat speed. 
for every millisecond the button is pressed the boat speed increases by 1 
millimiter per millisecond, starting from 0 millimeters per millisecond.

The milliseconds pressing the button to ramp up the boat speed count.


#### Extract from input

Extract:
- Time limit
- Distance to beat

from 0 to time limit press the button, calculate the distance and retrieve the
distance travelled. Only if distance travelled > distance to beat we increment
the count of ways to beat the record.