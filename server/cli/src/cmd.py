import argparse
from controller.src.utils import *
from config.parse_config import load_agents
from database.src.deploy import install_db, uninstall_db
from database.src.utils import es_read_agent_span_list
from trace.association.src.cross import inter_association
from trace.association.src import fifo, deeptrace, traceweaver_v1, traceweaver_v2, wap5, vpath
from trace.association.src.utils import span_merge
from database.src.utils import es_write_span_list
from trace.assemble.src.utils import assemble_trace_from_db


def parse_args():
    parser = argparse.ArgumentParser(description='参数解析示例')
    subparsers = parser.add_subparsers(dest='command', help='可用命令', required=True)

    # agent 子命令
    agent_parser = subparsers.add_parser('agent', help='agent相关操作')
    agent_subparsers = agent_parser.add_subparsers(dest='agent_action', help='agent操作', required=True)
    agent_subparsers.add_parser('install', help='安装agent')
    agent_subparsers.add_parser('test', help='测试连接agent')
    agent_subparsers.add_parser('stop', help='停止agent')
    agent_subparsers.add_parser('run', help='运行agent')

    # asso 子命令
    asso_parser = subparsers.add_parser('asso', help='asso相关操作')
    asso_subparsers = asso_parser.add_subparsers(dest='asso_action', help='asso操作', required=True)
    asso_algo = asso_subparsers.add_parser('algo', help='算法选择')
    asso_algo.add_argument('algorithm', choices=['fifo', 'deeptrace'], help='选择算法')

    # database 子命令
    db_parser = subparsers.add_parser('db', help='database操作')
    db_subparsers = db_parser.add_subparsers(dest='db_action', help='database操作', required=True)
    db_subparsers.add_parser('install', help='部署数据库')
    db_subparsers.add_parser('uninstall', help='卸载数据库')

    # assemble 子命令
    subparsers.add_parser('assemble', help='assemble操作')

    return parser.parse_args()

def main():
    args = parse_args()
    if args.command == 'agent':
        agents = load_agents()
        if args.agent_action == 'install':
            install_agents(agents)
        elif args.agent_action == 'stop':
            stop_agents(agents)
        elif args.agent_action == 'test':
            print("执行agent测试操作")
            test_agents(agents)
        elif args.agent_action == 'run':
            print("执行agent运行操作")
            start_agents(agents)
    elif args.command == 'asso':
        spans = es_read_agent_span_list()
        spans = inter_association(spans, client_ingress='ComposePost', tuple_used=False)
        if args.asso_action == 'algo':
            print(f"选择asso算法: {args.algorithm}")
            if args.algorithm == 'fifo':
                span_dict = fifo.fifo(spans)
        elif args.algorithm == 'deeptrace':
            span_dict = deeptrace.deeptrace(spans)
        elif args.algorithm == 'vpath':
            span_dict = vpath.vpath(spans)
        elif args.algorithm == 'wap5':
            span_dict = wap5.wap5(spans)
        elif args.algorithm == 'traceweaver_v1':
            span_dict = traceweaver_v1.traceweaver_v1(spans)
        elif args.algorithm == 'traceweaver_v2':
            span_dict = traceweaver_v2.traceweaver_v2(spans)
        else:
            raise ValueError(f"Unknown algorithm: {args.algorithm}")
        span_list = span_merge(span_dict)
        es_write_span_list(f'agent-span-mappings', span_list)
    elif args.command == 'assemble':
        print("执行assemble操作")
        assemble_trace_from_db('agent-span-mappings', 'agent-traces')
    elif args.command == 'db':
        if args.db_action == 'install':
            install_db()
        elif args.db_action == 'uninstall':
            uninstall_db()

if __name__ == '__main__':
    main()