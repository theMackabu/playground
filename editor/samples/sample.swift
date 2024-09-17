import UIKit

// MARK: - Model

struct Task: Codable, Identifiable {
	 let id: UUID
	 var title: String
	 var isCompleted: Bool
	 
	 init(id: UUID = UUID(), title: String, isCompleted: Bool = false) {
		  self.id = id
		  self.title = title
		  self.isCompleted = isCompleted
	 }
}

// MARK: - ViewModel

class TaskListViewModel: ObservableObject {
	 @Published var tasks: [Task] = []
	 
	 init() {
		  loadTasks()
	 }
	 
	 func addTask(_ title: String) {
		  let newTask = Task(title: title)
		  tasks.append(newTask)
		  saveTasks()
	 }
	 
	 func toggleTask(_ task: Task) {
		  if let index = tasks.firstIndex(where: { $0.id == task.id }) {
				tasks[index].isCompleted.toggle()
				saveTasks()
		  }
	 }
	 
	 func deleteTask(_ task: Task) {
		  tasks.removeAll { $0.id == task.id }
		  saveTasks()
	 }
	 
	 private func saveTasks() {
		  if let encoded = try? JSONEncoder().encode(tasks) {
				UserDefaults.standard.set(encoded, forKey: "savedTasks")
		  }
	 }
	 
	 private func loadTasks() {
		  if let savedTasks = UserDefaults.standard.data(forKey: "savedTasks"),
			  let decodedTasks = try? JSONDecoder().decode([Task].self, from: savedTasks) {
				tasks = decodedTasks
		  }
	 }
}

// MARK: - View

class TaskListViewController: UITableViewController {
	 private let viewModel = TaskListViewModel()
	 
	 override func viewDidLoad() {
		  super.viewDidLoad()
		  title = "Task List"
		  navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: .add, target: self, action: #selector(addNewTask))
		  
		  viewModel.$tasks
				.sink { [weak self] _ in
					 self?.tableView.reloadData()
				}
				.store(in: &cancellables)
	 }
	 
	 @objc private func addNewTask() {
		  let alert = UIAlertController(title: "New Task", message: "Enter a title for the new task", preferredStyle: .alert)
		  alert.addTextField { textField in
				textField.placeholder = "Task title"
		  }
		  let addAction = UIAlertAction(title: "Add", style: .default) { [weak self] _ in
				if let title = alert.textFields?.first?.text, !title.isEmpty {
					 self?.viewModel.addTask(title)
				}
		  }
		  alert.addAction(addAction)
		  alert.addAction(UIAlertAction(title: "Cancel", style: .cancel, handler: nil))
		  present(alert, animated: true)
	 }
	 
	 // MARK: - Table view data source
	 
	 override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
		  return viewModel.tasks.count
	 }
	 
	 override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
		  let cell = tableView.dequeueReusableCell(withIdentifier: "TaskCell", for: indexPath)
		  let task = viewModel.tasks[indexPath.row]
		  
		  var content = cell.defaultContentConfiguration()
		  content.text = task.title
		  content.secondaryText = task.isCompleted ? "Completed" : "Pending"
		  cell.contentConfiguration = content
		  
		  return cell
	 }
	 
	 // MARK: - Table view delegate
	 
	 override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
		  let task = viewModel.tasks[indexPath.row]
		  viewModel.toggleTask(task)
		  tableView.deselectRow(at: indexPath, animated: true)
	 }
	 
	 override func tableView(_ tableView: UITableView, commit editingStyle: UITableViewCell.EditingStyle, forRowAt indexPath: IndexPath) {
		  if editingStyle == .delete {
				let task = viewModel.tasks[indexPath.row]
				viewModel.deleteTask(task)
		  }
	 }
}

// MARK: - App Delegate

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
	 var window: UIWindow?
	 
	 func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
		  window = UIWindow(frame: UIScreen.main.bounds)
		  let navigationController = UINavigationController(rootViewController: TaskListViewController())
		  window?.rootViewController = navigationController
		  window?.makeKeyAndVisible()
		  return true
	 }
}