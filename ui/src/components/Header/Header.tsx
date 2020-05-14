import * as React from "react";
import { withRouter } from "react-router-dom";
import "./Header.css";
import Logo from "./img/logo.png";

const getTitle = (pathName: string) => {
  if (pathName === "/") {
    return "B-Harvest Dashboard";
  } else if (/^\/nodelist/.test(pathName)) {
    if (pathName === "/nodelist") {
      return "B-Harvest Node List";
    } else {
      return "B-Harvest Node Details";
    }
  } else if (pathName === "/rpc") {
    return "B-Harvest RPC";
  } else if (pathName === "/log") {
    return "B-Harvest Log";
  } else {
    return "B-Harvest";
  }
};

export const Header = withRouter(props => {
  return (
    <div className="header d-flex align-items-center">
      <div className="logo-container text-center">
        <img className="animated fadeIn logo" src={Logo} alt="" />
      </div>
      <div className="title-container text-center">
        <h3 className="animated fadeIn mb-0">
          {getTitle(props.location.pathname)}
        </h3>
      </div>
      <div>
        <span>{process.env.REACT_APP_TITLE}</span>
      </div>
      <div className="option-container" />
    </div>
  );
});
