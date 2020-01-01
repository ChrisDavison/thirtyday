# thirtyday

## Usage

This outputs a simple 30-day 'checkbox' for use in a 30-day challenge. For
example, if used today (Wed, 1st January, 2020): 

    thirtyday

generates...

    [WTFSS MTWTFSS MTWTFSS MTWTFSS MTWT]

## Purpose

The purpose of this is so that if you embark on a new skill, or a new habit,
you can track it in plaintext.  Throughout the challenge, you replace the
letter of the day with an 'X' for success, or '.' for fail, and then you can
see at the end of the month something like:

    [xxxxx xxxx..x xx.x..x xxxx.xx xxxx]

We can then see that we didn't achieve our goal on certain days.

This is generally intended to be used within a file (e.g. you create a text
document and insert the template), which you then fill with notes about your
challenge.

My current template is:

```markdown
# Title

    <thirtyday-output>

TAGS

WHAT I WANTED TO DO

WHAT I DID

WHAT I WASN'T ABLE TO DO

WHAT WENT RIGHT

WHAT WENT WRONG

IDEAS FOR IMPROVEMENT
```

I can then reflect throughout the month, and at the end of the month, to make
plans for future progress.
