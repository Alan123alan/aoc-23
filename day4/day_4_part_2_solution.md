#### Part 2 question

Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, _how many total scratchcards do you end up with?_

###### Test input

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

###### Understanding card copying process

- Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5. `first copy`

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

- Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4. `second copy`

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

- Your copy of card 2 also wins one copy each of cards 3 and 4. `third copy`

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

- Your four instances of card 3 (one original and three copies) have two matching numbers, so you win _four_ copies each of cards 4 and 5. `fourth copy`

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

###### Coming up with Rules

1. The card you are currently on doesn't get copied
2. For the number of copies (equal to the wins per your current card) you add 1 to the count of cards you have for each card number after your current card.
3. The previous step gets repeated for the current card count (original + copies already processed)

###### Vectors and for loops

Make a vector keeping track of the count of cards for each card number.

[1, 1, 1, 1, 1 , 1] == [card #1 count, card #2 count, card #3, card #4 count, card #5 count , card #6 count]

Translating rules to pseudocode:

for c in 0..card_count
	for i in (index+1)..(offset+#copies)
		vec[i] += 1

The offset should be the start point (index + 1).

- Check card 1 for winning numbers, it has 4 winning numbers, so you get copies for cards #2, #3, #4 and #5

```
	for c in 0..1
		for i in (0+1)..((0+1)+#copies)
			vec[i] += 1
```
	
	After first copy:
	
	[1, 2, 2, 2, 2, 1]

- Check card #2 (original) for winning numbers, it has 2 winning numbers, so you get copies for card  #3 and #4 (copying twice since current count for card #2 == 2)

```
	for c in 0..2
		for i in (1+1)..((1+1)+#copies)
			vec[i] += 1
```
	
After first iteration (same as after second copy):
	
[1, 2, 3, 3, 2, 1]
	
After second iteration (same as after third copy):
	
[1, 2, 4, 4, 2, 1]