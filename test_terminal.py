import webbrowser
import os
import time
import requests
from datetime import datetime
import sys
from rich.console import Console
from rich.markdown import Markdown
import subprocess
import rich

from prompt_toolkit import PromptSession
from prompt_toolkit.history import FileHistory
from prompt_toolkit.styles import Style

# 初始化控制台
console = Console()

# 创建带历史记录的会话
session = PromptSession(history=FileHistory('.command_history'))

# 定义提示符样式 (对应你的 Rich 样式)
def get_prompt():
    return [
        ('#00ff00 bold', 'What '),      # bright_green
        ('#ffff00 bold', 'you '),       # bright_yellow
        ('#0000ff bold', 'want to '),   # bright_blue
        ('#ff00ff bold', 'do '),        # agent
        ('#ff00ff', '➜ '),             # arrow
    ]

# 绘制 ASCII 评级图表 (保留原有功能)
def draw_ascii_chart(data, width=60, height=20):
    if not data:
        return ""

    ratings = [x[3] for x in data]
    dates = [datetime.fromtimestamp(x[2]).strftime("%Y-%m") for x in data]

    min_rating = min(ratings)
    max_rating = max(ratings)
    range_rating = max_rating - min_rating
    if range_rating == 0:
        range_rating = 1

    normalized = [int((r - min_rating) / range_rating * (height - 1)) for r in ratings]

    chart = []
    for y in range(height - 1, -1, -1):
        line = []
        for i, val in enumerate(normalized):
            if i >= width:
                break
            if val == y:
                line.append("●")
            elif val > y:
                line.append("│")
            else:
                line.append(" ")

        current_rating = int(min_rating + (y / (height - 1)) * range_rating)
        line.append(f" {current_rating}")
        chart.append("".join(line))

    x_labels = []
    step = max(1, len(dates) // width)
    for i in range(0, min(len(dates), width), step):
        x_labels.append(dates[i][-2:])

    chart.append(" ".join(x_labels))

    return "\n".join(chart)

# 显示评级历史 (保留原有功能)
def show_rating_history(username):
    try:
        url = f"https://codeforces.com/api/user.rating?handle={username}"
        response = requests.get(url)
        data = response.json()
        if data["status"] == "OK":
            rating_data = [
                (c["contestId"], c["contestName"], c["ratingUpdateTimeSeconds"], c["newRating"])
                for c in data["result"]
            ]
        else:
            console.print("[bright_red]Error fetching rating data:[/]", data.get("comment", "Unknown error"))
            return
    except Exception as e:
        console.print("[bright_red]Error:[/]", e)
        return

    console.print(f"[bright_green]\nCodeforces Rating History ({username}):[/]")
    console.print("[bright_yellow]=[/]" * 70)

    current_rating = rating_data[-1][3]
    max_rating = max(r[3] for r in rating_data)
    min_rating = min(r[3] for r in rating_data)
    contests_count = len(rating_data)

    console.print(f"[bright_red]Current Rating: {current_rating}[/]")
    console.print(f"[bright_green]Highest Rating: {max_rating}[/]")
    console.print(f"[bright_yellow]Lowest Rating: {min_rating}[/]")
    console.print(f"[bright_blue]Contests Participated: {contests_count}[/]")
    console.print("[bright_magenta]\nRating Chart:[/]")

    chart = draw_ascii_chart(rating_data)
    if chart:
        console.print(chart)

    console.print("[bright_red] \nLast 5 Contests: [/]")
    for contest in rating_data[-5:]:
        date = datetime.fromtimestamp(contest[2]).strftime("%Y-%m-%d")
        console.print(f"[bright_green]{date}: {contest[1]} (Rating: {contest[3]})[/]")

# 主函数
def main():
    # 启动动画
    console.print("[bold bright_cyan]Welcome to XTerminal![/]")
    console.print("[bright_yellow]Type 'help' for available commands[/]")
    
    while True:
        try:
            # 使用 prompt_toolkit 获取输入
            command = session.prompt(get_prompt())
            
            # 处理命令
            if command == "home":
                webbrowser.open("https://atcoder.jp/")
                console.print("[green]Opened AtCoder home page[/]")

            elif command == "contest":
                webbrowser.open("https://atcoder.jp/contests/")
                console.print("[green]Opened contests page[/]")

            elif command == "rank":
                webbrowser.open("https://atcoder.jp/ranking")
                console.print("[green]Opened global rankings[/]")

            elif command == "userdata":
                username = input("Enter your username: ")
                webbrowser.open(f"https://atcoder.jp/users/{username}")
                console.print(f"[green]Opened profile for {username}[/]")

            elif command == "play":
                contest_name = input("Enter contest name: ")
                webbrowser.open(f"https://atcoder.jp/contests/{contest_name}")
                console.print(f"[green]Opened contest: {contest_name}[/]")

            elif command == "clear":
                os.system("cls" if os.name == "nt" else "clear")

            elif command == "time":
                console.print(time.strftime("[bright_red]%Y-%m-%d %H:%M:%S[/]", time.localtime(time.time())))

            elif command == "help":
                console.print("[bright_yellow]\nAvailable commands:[/]")
                console.print("[bright_yellow]home       - Open AtCoder home page[/]")
                console.print("[bright_yellow]contest    - Open contests page[/]")
                console.print("[bright_yellow]rank       - Open global rankings[/]")
                console.print("[bright_yellow]userdata   - View user profile[/]")
                console.print("[bright_yellow]play       - Open specific contest[/]")
                console.print("[bright_yellow]clear      - Clear terminal screen[/]")
                console.print("[bright_yellow]time       - Show current time[/]")
                console.print("[bright_yellow]rating     - Show Codeforces rating graph[/]")
                console.print("[bright_yellow]agent      - Start AI assistant[/]")
                console.print("[bright_yellow]exit       - Exit the program[/]")
                console.print("[bright_yellow]help       - Show this help message\n[/]")

            elif command == "rating":
                username = console.input("[bright_white]Enter Codeforces username: [/]")
                show_rating_history(username)

            elif command == "agent":
                console.print("[bold cyan]Starting AI assistant... Type 'quit' to exit[/]")
                agent_session = PromptSession(history=FileHistory('.agent_history'))
                
                while True:
                    agent_prompt = [
                        ('#ff6e4a bold', 'Ollama'),  # prompt
                        ('#00ffff italic', ' XTerminal Agent'),  # agent
                        ('yellow blink', ' ➜ '),  # arrow
                    ]
                    
                    problem = agent_session.prompt(agent_prompt)
                    
                    if problem.lower() == "quit":
                        break
                    
                    try:
                        # 流式输出
                        process = subprocess.Popen(
                            ["ollama", "run", "llama3", problem],
                            stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE,
                            text=True,
                            bufsize=1,
                            universal_newlines=True
                        )
                        
                        for line in process.stdout:
                            rich.print(Markdown(line), end="")
                    
                    except Exception as e:
                        console.print(f"[red]Error: {e}[/]")

            elif command == "exit":
                confirm = console.input("[bright_red]Are you sure to exit? (y/n): [/]")
                if confirm.lower() == "y":
                    console.print("[green]Goodbye![/]")
                    break

            elif command.strip() == "":
                continue

            else:
                console.print("[cyan]Command not found. Type 'help' for available commands.[/]")

        except KeyboardInterrupt:
            console.print("\n[red]Command cancelled[/]")
            continue
        except EOFError:
            console.print("\n[green]Exiting...[/]")
            break

if __name__ == "__main__":
    main()