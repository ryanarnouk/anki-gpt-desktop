import NotesPage from "./pages/NotesPage";
import AnswerPage from "./pages/AnswerPage";
import { BrowserRouter as Router, Route, Routes, Link } from "react-router-dom";
import ApiKeyPage from "./pages/ApiKeyPage";

function App() {
  return (
    <div>
      <Router>
        <Routes>
          <Route path="/add" Component={NotesPage}/>
          <Route path="/" Component={AnswerPage} />
          <Route path="/update" Component={ApiKeyPage} />
        </Routes>
        <ul>
          <li>
            <Link to="/">Answer Questions</Link>
          </li>
          <li>
            <Link to="/add">Add New Notes</Link>
          </li>
          <li>
            <Link to="/update">Update Environment Variable</Link>
          </li>
        </ul>
      </Router>
    </div>
  );
}

export default App;
