# A Somewhat Stream of Consciousness set of Notes made while I'm not able to actually work on the code

## Disclaimer: I am actually kinda still reading the rust book while I build this.

- division function needs to end in an expression (no semi-colon) which is a tuple (result, remainder), benefit of this is that I *should* be able to call the div function recursively until the remainder == 0. 

work in the opposite direction. instead of halving the goal weight, double the increments available and then half those later. 

eg. 40kg + Bar weight, don't half to 20kg and then divide by 25 kg then 20 kg then 15 kg etc. divide the 40 kg by 50 kg then 40 kg then 30 kg and afterwards, call those 2x25, 2x20, 2x15, 2x10 etc...


div(weight, increment):
while {remainder} > {increment} :
	blah blah blah
	return (result, remainder)

find_plates(weight) -> ? {
	remaining_weight = weight
	vec= []
	for len(plates):
		div(remaining_weight, plates[0])
		append (result) to vec
		remaining_weight = remainder
}

worked example:

137.5 + bar:

div(137.5,50):
returns: (2, 37.5)
div(37.5,40):
returns: (0,37.5)
div(37.5,30):
returns: (1, 7.5)

etc...

ultimate this examples final should be: [2 sets of 50, 1 set of 30, 1 set of 5 and 1 set of 2.5] == [25+25+15+2.5+1.25] on each side of the bar. 

Available plate is ultimately something like a Dict or an array/vector of tuples (is a tuple-vec even a thing?) -- some kind of Key:Value pair, because not everyone has a
functionally unlimited amount of every (standard) weight plate available to them. *I* do, but that's because I work out at, like, 4am in a 24/7 gym where there's not that
many people around.

Imperial Plates (lbs): [55,45… 5]
Metric Plates (kgs): [25, 20, 15, 10, 5, 2.5, 1.25]

Can access values from tuple functions via:

let (a, _) = tuple_fn(); -- This one makes it clear that we are unpacking the tuple and deliberately discarding it and is scalable.

or

let a = tuple_fn().0; 
