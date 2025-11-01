from config.parse_config import load_agents



if __name__ == "__main__":
    agents = load_agents()
    for name, agent in agents.items():
        print(f"Agent Name: {name}")
        print(f' server_config: {agent.server_config}')
        print(f' agent_config: {agent.agent_config}')
        print(f' elastic_config: {agent.elastic_config}')
        print(f' code_path: {agent.code_path}')
        print(f' agent_info: {agent.agent_info}')
    print(f"Total agents loaded: {len(agents)}")