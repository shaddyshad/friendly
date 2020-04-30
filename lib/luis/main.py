from modules.resolve import resolve_input    

if __name__ == "__main__":
    while 1:
        text = input(":> ")

        if text == "q":
            break

        res = resolve_input(text)
        print(res)
    