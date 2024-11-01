import tkinter as tk
from tkinter import filedialog


class App(tk.Frame):
    def __init__(self, master=None):
        super().__init__(master)
        self.pack()


root = tk.Tk()

# create the application
myapp = App(root)

#
# here are method calls to the window manager class
#
root.title("My Do-Nothing Application")
root.maxsize(1000, 400)

root.update()
root.minsize(1000, 400)

# start the program
myapp.mainloop()
# file_path = filedialog.askopenfilename()
