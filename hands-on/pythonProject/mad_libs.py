def get_input(word_type: str):
    user_input: str = input(f"Enter a {word_type}:")
    return user_input


noun_one = get_input("noun")
adjective = get_input("adjective")
verb_one = get_input("verb")
noun_two = get_input("noun")
verb_two = get_input("verb")

story = f"""
Once upon a time, there was a {adjective} {noun_one} who loved to {verb_one} all day.

One day, {noun_two} walked into the room and caught the {noun_one} in the act. 

{noun_two}: "What are you doing?"
{noun_one}: "I'm just {verb_one}ing, what's the big deal?"
{noun_two}: "Well, it's not every day that you see a {noun_one} {verb_one}ing in the middle of the day."
{noun_one}: "I guess you're right. Maybe I should take a break."
{noun_two}: "That's probably a good idea. Why don't we go {verb_two} instead?"
{noun_one}: "Sure, that sounds like fun!"

And so, {noun_two} and the {noun_one} went off to {verb_two} and had a great time. 
The end.
"""

print(story)
