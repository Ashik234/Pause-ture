export type Quip = { kind: "joke" | "fact"; text: string };

export const QUIPS: Quip[] = [
  // jokes
  { kind: "joke", text: "I told my computer I needed a break. It said: no problem — I'll go to sleep too." },
  { kind: "joke", text: "Why do programmers prefer dark mode? Because light attracts bugs." },
  { kind: "joke", text: "A SQL query walks into a bar, approaches two tables and asks: mind if I join you?" },
  { kind: "joke", text: "There are 10 types of people: those who understand binary and those who don't." },
  { kind: "joke", text: "99 little bugs in the code. Take one down, patch it around — 127 little bugs in the code." },
  { kind: "joke", text: "Why did the developer go broke? He used up all his cache." },
  { kind: "joke", text: "!false — it's funny because it's true." },
  { kind: "joke", text: "A programmer's partner says: buy a loaf of bread, and if they have eggs, buy a dozen. He returns with 12 loaves." },
  { kind: "joke", text: "Hardware: the part of the computer you can kick." },
  { kind: "joke", text: "My code doesn't have bugs. It has undocumented features." },
  { kind: "joke", text: "Why do Java developers wear glasses? Because they don't C#." },
  { kind: "joke", text: "To understand recursion, you must first understand recursion." },
  { kind: "joke", text: "\"It works on my machine\" — famous last words." },
  { kind: "joke", text: "Debugging: being the detective in a crime movie where you are also the murderer." },
  { kind: "joke", text: "The best thing about a Boolean: even if you're wrong, you're only off by a bit." },
  { kind: "joke", text: "A UDP packet walks into a bar. Nobody acknowledges it." },
  { kind: "joke", text: "I would tell you a joke about DNS, but it might take 24 hours to reach you." },
  { kind: "joke", text: "Programming is 10% writing code and 90% figuring out why it doesn't work." },

  // facts
  { kind: "fact", text: "You blink about 66% less while staring at a screen — that's why your eyes feel dry." },
  { kind: "fact", text: "The 20-20-20 rule exists because your eye's focusing muscle never rests at close range." },
  { kind: "fact", text: "Your brain is ~75% water. Just 2% dehydration measurably drops concentration." },
  { kind: "fact", text: "An hour of sitting reduces blood flow to the brain. A 2-minute walk restores it." },
  { kind: "fact", text: "Your spine is about 1 cm shorter in the evening than in the morning." },
  { kind: "fact", text: "Looking at something far away fully relaxes the ciliary muscle inside your eye." },
  { kind: "fact", text: "A 5-minute walk every hour offsets most of the metabolic harm of sitting." },
  { kind: "fact", text: "Your head weighs ~5 kg upright — but pulls like 27 kg on your neck at a 60° phone tilt." },
  { kind: "fact", text: "Drinking water improves reaction time within about 15 minutes." },
  { kind: "fact", text: "Standing burns roughly 50 more calories per hour than sitting." },
  { kind: "fact", text: "Walking boosts creative thinking by around 60% — Stanford measured it." },
  { kind: "fact", text: "Muscles begin stiffening after just 30 minutes of stillness." },
  { kind: "fact", text: "Office workers spend up to 1,700 hours a year looking at screens." },
  { kind: "fact", text: "Good hydration keeps the discs between your vertebrae cushioned — they're mostly water." },
  { kind: "fact", text: "Natural light exposure during breaks helps regulate your sleep cycle." },
  { kind: "fact", text: "Slouching can reduce lung capacity by up to 30%." },
];

export function randomQuip(): Quip {
  return QUIPS[Math.floor(Math.random() * QUIPS.length)];
}
