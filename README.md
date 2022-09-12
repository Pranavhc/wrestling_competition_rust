This is my first rust project, it's pretty basic but taught me some real rust stuff. 
I believe going through this might help beginners to think in "the rust way".

This is what i could learn -

  1. Overnership and borrowing 
      - two immutable refference of same obj cannot exist! keep it in mind.
      
  2. Lifetime 
      - feels very difficult at first but it isn't.
      
  3. Mutability in Immutable object is possible
      - by using std::cell::Cell or RefCell, we can make a feild mutable even if the object itself is immutable.
