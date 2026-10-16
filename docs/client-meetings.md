# Client meetings

TWA's rules come from people: the training manager, HR, station managers, the regulator's documents. Your job in a meeting is to turn what they say into rules exact enough to code and test, and to explain what you've built without making anyone feel slow.

## Before the meeting

- **Know the one decision you need from this meeting.** Write it at the top of your notes.
- **Bring examples, not abstractions.** A fake staff member with a real-looking gap report beats a diagram.
- **Prepare the questions below** for whatever qualification or feature is on the agenda.

## Turning a requirement into a rule

When someone says *"Dangerous Goods is valid for two years"*, ask until there's nothing left to guess:

| Ask | Why it matters |
| --- | --- |
| Two years to the day, or to the end of the month? | Changes expiry dates by up to 30 days (0x08) |
| Counted from the course date, the exam date, or the date the certificate was issued? | Changes which date the system stores |
| Is there a grace period after expiry? Can someone in grace work? | `InGrace` vs `Expired` (0x04, 0x09) |
| Must it be *used* to stay current (recency)? How do we know when it was last used? | `Lapsed`, and a data source you may not have yet |
| Does a refresher extend it? From when? | A different rule from the initial course |
| Is any other certificate accepted instead? | Equivalents (0x09) |
| What must someone hold first? | Prerequisites (0x09) |
| Who decides, and where is it written down? | The source you'll cite in the code and decision records |

Then **write each answer as a test case** in the fixtures file, with fake data, and send the cases back to the client in plain words: *"Amina completed DG on 10 March 2025. On 20 March 2027 she is: in grace / expired / valid. Correct?"* People who can't read code can always check examples.

## Explaining what you've built

- **Start with what they'll see**, not how it works: "On Monday morning, everyone whose fire safety expires within a month gets an SMS."
- **One idea per sentence.** No acronyms they didn't use first.
- **Say what the system won't do** as clearly as what it will: "The app can show your status offline, but only the training office can record a new certificate."
- **When you don't know, say so** and give a date for the answer.

## A 30-minute demo script

| Minutes | What |
| --- | --- |
| 0–3 | What we're showing today, and the one decision we need from you |
| 3–13 | The demo: a transfer gap check for a fake person, then the 06:00 alerts. Explain each line on screen in plain words |
| 13–23 | Open questions, from the worksheet above, one at a time. Write the answers down live |
| 23–28 | Read back the answers as rules and examples. Fix anything misheard |
| 28–30 | Next steps, owners and dates |

## After the meeting

Send a short written summary the same day: the decisions, the open questions, and the example cases. Anything decided goes into a decision record. Anything unclear stays an open question, in writing, with an owner.
