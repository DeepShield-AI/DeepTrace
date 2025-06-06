
from config.parse_config import load_agents
from controller.src.utils import *


if __name__ == '__main__':
    agents = load_agents()
    test_agents(agents)