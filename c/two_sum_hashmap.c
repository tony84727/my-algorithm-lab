#include <stdlib.h>
struct Entry {
	int key;
	void* value;
};

struct Map {
	int (*hash)(int);
	struct Entry*** slots;
	int* slot_lengths;
	int slot_size;
};
struct Map* hashmap_init(int (*hash_function)(int), int slot_size) {
	struct Map* m = malloc(sizeof(struct Map));
	m->hash = hash_function;
	m->slots = calloc(slot_size, sizeof(struct Entry*));
	m->slot_lengths = calloc(slot_size, sizeof(int));
	m->slot_size = slot_size;
	return m;
}

static void hashmap_add(struct Map* map, int key, void* item) {
	int hash_value = map->hash(key);
	if (map->slot_lengths[hash_value] == 0) {
		map->slots[hash_value] = calloc(2, sizeof(struct Entry**));
	}
	struct Entry* entry = malloc(sizeof(struct Entry));
	entry->key = hash_value;
	entry->value = item;
	map->slots[hash_value][map->slot_lengths[hash_value]] = entry;
	map->slot_lengths[hash_value] += 1;
	if ((map->slot_lengths[hash_value] & map->slot_lengths[hash_value] - 1) == 0)
	{
		map->slots[hash_value] = realloc(
			map->slots[hash_value],
			map->slot_lengths[hash_value] * 2 * sizeof(struct Entry**));
	}
}

static void* hashmap_get(struct Map* map, int needle) {
	int hash_value = map->hash(needle);
	for(int i = 0; i < map->slot_lengths[hash_value]; i++) {
		struct Entry* entry = map->slots[hash_value][i];
		if (entry->key == needle) {
			return entry->value;
		}
	}
	return NULL;
}

static const int SLOT_SIZE = 1024 * 1024;

static int mod(int i) {
	return (i % SLOT_SIZE + SLOT_SIZE) % SLOT_SIZE;
}

int* two_sum(int* nums, int num_size, int target, int* return_size) {
	struct Map* m = hashmap_init(&mod, SLOT_SIZE);
	for(int i = 0; i < num_size; i+=1) {
		void* index = malloc(sizeof(int));
		*((int*)index) = i;
		hashmap_add(m, nums[i], index);
	}
	for (int i = 0; i < num_size; i+=1) {
		int remaining = target - nums[i];
		if (remaining < 0) {
			continue;
		}
		int* index = hashmap_get(m, remaining);
		if (index == NULL || *index == i) {
			continue;
		}
		*return_size = 2;
		int* matches = calloc(2, sizeof(int));
		matches[0] = i;
		matches[1] = *index;
		return matches;
	}
	return NULL;
}
