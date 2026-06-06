const adjectives = [
  "Eternal",
  "Crimson",
  "Shadow",
  "Neon",
  "Ancient",
  "Rogue",
  "Cyber",
  "Dark",
  "Cosmic",
  "Cozy",
  "Last",
  "Animal",
  "Rising",
  "Awakening",
];

const nouns = [
  "Quest",
  "Realm",
  "Galaxy",
  "Chronicles",
  "Dungeon",
  "Saga",
  "Legion",
  "Frontier",
  "Nexus",
  "Abyss",
  "Cafe",
  "Forest",
  "Village",
  "City",
  "Kingdom",
];

export function randomGameTitle(): string {
  const pick = <T>(arr: T[]) => arr[Math.floor(Math.random() * arr.length)];
  return `${pick(adjectives)} ${pick(nouns)}`;
}
