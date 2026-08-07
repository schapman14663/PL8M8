# PL8M8

- A Small Gadget For Working Out Which Weight Plates to add to your Barbell.

Ultimately wanting to put this into the (currently not really started), LFTR
project that aims to be able to help me with my lifting programming. The idea
being that given an amount of available plates in the gym and an amount of weight
to be added to the barbell (for now?) PL8M8 would:

- round to the nearest multiple of whatever your programming says your increment
for weight is, this is usually 2.5kg or 5lbs for barbell activities.
- determine how many of each sized plate that is available to add to the bar overall.

For example, if you needed to add 60kg to the bar, and you had 1 25kg plate,
2 20kg plates, 2 15kg plates , 2 10kg plates, 2 5kg plates and 2 2.5kg plates,
you would be instructed to use both the 20kg plates and both the 10kg plates
with 1 of each going on either side of the barbell.

For this, I have implemented a simple division as I want to use both the result
and the remainder, and this will also give me control over the plate sorting
algorithm.

I currently am having some issues with exactly how to go about implementing parts
of this, a HashMap does not currently seem to be the best idea, and implementing
a struct also does not feel like the right idea because you can't have a struct
with a struct in it in rust so, while I might be able to implement a struct for
PL8M8, this will likely cause issues for LFTR later.

I am intending on trying to change to a tuples based system over the next few
days, mostly because I don't want to delve into using an external crate at the
moment, which is something I think would need to be done in order to have a
HashMap with floats in it. (both Metric and Imperial weight plate sets have
plates that prevent the use of unsigned integers, i.e. both 2.5 kg and 2.5 lbs
plates do exist and need to be usable, the gym I go to personally has 1.25 kg
plates) because you can't use a float as a key since they don't implement Eq and
aren't Hashable which I think is because of the way bit math works.

TO DO:

- ~~re-organise how the available plates would be stored in memory.~~
- ~~change how the division tuple is represented (w, r) to maybe something more
descriptive like (res, rem).~~
- ~~implement function to obtain a list of plates that are available to the user.~~
- implement a function to sort those plates.
- start work on functions that allow a user to add weight to the bar.
- tidy the code (there's a lot of suggestions under `cargo check` at this time)
- implement generics and reduce reliance on things like `<f64> as u32`
- implement a logger
- improve debugging

Current Issues (7 Aug 26)

- There is an issue with `get_rounded_weight` where it will not recur correctly
- The `plate_sort` function does not seem to actually perform its loop for some
reason
