
from config.parse_config import load_agents
from controller.src.utils import *


if __name__ == '__main__':
    # agents = load_agents()
    # test_agents(agents)
    # get_all_k8s_tags()
    agents = load_agents()
    for name, agent in agents.items():
        # agent.clone_code({})
        agent.get_pids()
        agent.sync_config()
        agent.run()