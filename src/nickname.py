# This whole thing was transposed from Rust by ChatGPT

import random
import string
from multiprocessing import cpu_count, Pool

# Dataset from https://data.world/alexandra/baby-names
with open('./names.txt') as f:
    NAMES = f.read().replace('\r', '')

INSERTS_TO_TRY = 100_000


class Nickname:
    """
    Nickname is a random name generator
    It appends a random number of trailing characters to a random name
    """

    def __init__(self, trailing_chars: int) -> None:
        self.names = NAMES.split('\n')
        self.trailing_chars = trailing_chars

    def name(self) -> str:
        i = random.randint(0, len(self.names)-1)
        name = self.names[i]
        """
        if name[0] == '/':
            name = self.name()
        """
        name += f"-{self.random_chars(self.trailing_chars)}"
        return name

    def random_chars(self, number: int) -> str:
        return ''.join(random.choices(string.ascii_letters + string.digits, k=number))


def nickname_test() -> int:
    available_parallelism = cpu_count()
    pool = Pool(processes=available_parallelism)
    namer = Nickname(5)
    names = {}
    i = 0
    for results in pool.imap_unordered(_nickname_worker, [namer]*available_parallelism):
        for name, count in results.items():
            i += 1
            print(f'{i}/{INSERTS_TO_TRY/available_parallelism}')
            if name in names:
                count += names[name]
            names[name] = count

    collisions = {}
    for name, count in names.items():
        if count > 1:
            collisions[name] = count

    print(f'Test finished with {len(collisions)} collisions')
    return len(collisions)


def _nickname_worker(namer: Nickname) -> dict:
    names = {}
    collision_count = 0
    for n in range(INSERTS_TO_TRY):
        name = namer.name()
        if name in names:
            count = names[name]
            count += 1
            names[name] = count
            collision_count += 1
        else:
            names[name] = 1

    print(f'{collision_count} collisions')
    return names


if __name__ == '__main__':
    nickname_test()
