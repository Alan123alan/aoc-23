#### Initial State
initial position at the start of the map, index == 0
initialize steps = 0
#### Loop
read the instruction (can be either L or R)
if L take the value to the left and set it to the new index
if R take the value to the right and set it to the new index
increment steps by 1
if new position id is "ZZZ" break out