from os.path import dirname


def run():
    with open(f'{dirname(__file__)}/data.txt', 'r') as file:
        lines = file.read().split('\n')[:-1]


if __name__ == '__main__':
    print(run())
