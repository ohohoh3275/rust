additional notes

### reference

- immutable reference == shared reference : &
- mutable reference == unique reference : &mut

immutable reference and mutable reference cant be compatible.

<html>
Because using & is called "referencing", using * is called "dereferencing".

Rust has two rules for mutable and immutable references. They are very important, but also easy to remember because they make sense.

Rule 1: If you have only immutable references, you can have as many as you want. 1 is fine, 3 is fine, 1000 is fine. No problem.

Rule 2: If you have a mutable reference, you can only have one. Also, you can't have an immutable reference and a mutable reference together.

This is because mutable references can change the data. You could get problems if you change the data when other references are reading it.

<u>
A good way to understand is to think of a Powerpoint presentation.
</u>

Situation one is about only one mutable reference.

Situation one: An employee is writing a Powerpoint presentation. He wants his manager to help him. The employee gives his login information to his manager, and asks him to help by making edits. Now the manager has a "mutable reference" to the employee's presentation. The manager can make any changes he wants, and give the computer back later. This is fine, because nobody else is looking at the presentation.

Situation two is about only immutable references.

Situation two: The employee is giving the presentation to 100 people. All 100 people can now see the employee's data. They all have an "immutable reference" to the employee's presentation. This is fine, because they can see it but nobody can change the data.

<b>Situation three is the problem situation.</b>

Situation three: The Employee gives his manager his login information. His manager now has a "mutable reference". Then the employee went to give the presentation to 100 people, but the manager can still login. This is not fine, because the manager can log in and do anything. Maybe his manager will log into the computer and start typing an email to his mother! Now the 100 people have to watch the manager write an email to his mother instead of the presentation. That's not what they expected to see.

</html>

https://dhghomon.github.io/easy_rust/Chapter_17.html
