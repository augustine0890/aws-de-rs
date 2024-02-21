from random import randint


def roll_dice(times: int = 2) -> list[int]:
    if times <= 0:
        raise ValueError
    rolls: list[int] = []
    for i in range(times):
        random_roll: int = randint(1, 6)
        rolls.append(random_roll)
    return rolls


def main():
    while True:
        try:
            user_input: str = input('How many dice would you like to roll? ')
            if user_input.lower() == 'exit':
                print('Thanks for playing!')
                break

            print(*roll_dice(int(user_input)), sep=', ')
        except ValueError:
            print('Please enter a valid number')


if __name__ == '__main__':
    main()
