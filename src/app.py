from flask import Flask, render_template, request
from io import StringIO
import menu
import sys

app = Flask(__name__, template_folder='templates')

@app.route('/')
def index():
	return render_template('index.html')

@app.route('/process_input', methods=['GET', 'POST'])
def process_input():
	if request.method == 'POST':
		user_input = request.form['user_input']
		result = process_user_input(user_input)
		return render_template('result.html', result=result)
	else:
		# Handle GET requests (if needed)
		return "Unsupported method: GET"

def process_user_input(user_input):
	# Redirect sys.stdout to capture console output
	old_stdout = sys.stdout
	new_stdout = StringIO()
	sys.stdout = new_stdout
	# Call the menu function with the user input
	menu.menu(user_input)
	# Get the captured console output
	console_output = new_stdout.getvalue()
	# Replace newline characters with HTML line breaks
	console_output_html = console_output.replace('\n', '<br>')
	# Restore sys.stdout
	sys.stdout = old_stdout

	return console_output_html

if __name__=='__main__':
	app.run(debug=True, host='0.0.0.0', port=8000)