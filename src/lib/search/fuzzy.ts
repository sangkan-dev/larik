export type FuzzySearchItem = {
  id: string;
  title: string;
  subtitle?: string;
  shortcut?: string;
};

export type FuzzySearchResult<T extends FuzzySearchItem> = T & {
  score: number;
};

export function fuzzySearch<T extends FuzzySearchItem>(
  items: T[],
  query: string,
) {
  const normalizedQuery = normalize(query);

  if (!normalizedQuery) {
    return items.map((item, index) => ({ ...item, score: -index }));
  }

  return items
    .map((item) => {
      const haystack = normalize(`${item.title} ${item.subtitle ?? ""}`);
      const score = scoreMatch(haystack, normalizedQuery);
      return score === null ? null : { ...item, score };
    })
    .filter((item): item is FuzzySearchResult<T> => Boolean(item))
    .sort((left, right) => right.score - left.score);
}

function scoreMatch(haystack: string, query: string) {
  let cursor = 0;
  let score = 0;
  let streak = 0;

  for (const character of query) {
    const index = haystack.indexOf(character, cursor);

    if (index === -1) {
      return null;
    }

    streak = index === cursor ? streak + 1 : 0;
    score += 10 + streak * 4 - index;
    cursor = index + 1;
  }

  return score - haystack.length * 0.01;
}

function normalize(value: string) {
  return value.trim().toLowerCase();
}
