import React, { useState } from "react";
import ReactModal from "react-modal";
import SplashPage from "./pages/SplashPage";
import McpServersPage from "./pages/McpServersPage";
import GuardProfilesPage from "./pages/GuardProfilesPage";
import ServerCollectionsPage from "./pages/ServerCollectionsPage";
import PendingMessagesPage from "./pages/PendingMessagesPage";
import { ToastContainer } from "react-toastify";
import "./App.css";

const PAGES = [
  {
    label: "MCP Guardian",
    component: SplashPage,
  },
  {
    label: "MCP Servers",
    component: McpServersPage,
  },
  {
    label: "Guard Profiles",
    component: GuardProfilesPage,
  },
  {
    label: "Server Collections",
    component: ServerCollectionsPage,
  },
  {
    label: "Pending Messages",
    component: PendingMessagesPage,
  },
];

ReactModal.setAppElement("#root");

const App = () => {
  const [pageIndex, setPageIndex] = useState(0);

  return (
    <main>
      <div className="main-grid">
        <div className="nav-link-container">
          {PAGES.map((page, i) => (
            <div
              key={`nav-link-${i}`}
              className={pageIndex === i ? "nav-link active" : "nav-link"}
              onClick={() => setPageIndex(i)}
            >
              {page.label}
            </div>
          ))}
        </div>
        <div>
          <div className="page">{React.createElement(PAGES[pageIndex].component)}</div>
        </div>
      </div>
      <ToastContainer />
    </main>
  );
};

export default App;
