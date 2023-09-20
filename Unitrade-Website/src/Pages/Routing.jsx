import React from "react";
import Home from "./Home";
import Login from "./Login";
import Signup from "./Signup";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Products from "../components/Products";
import Store from "./Store";

function Routing () {
return (
      <div>
        <Router>
            <Routes>
                <Route index element = {<Home />}/>
                <Route path="/home" element = {<Home />}/>
                <Route path="login" element = {<Login />}/>
                <Route path="signup" element = {<Signup />}/>
                <Route path="products" element = {<Store />}/>
            </Routes>
        </Router>
    </div>
);
}

export default Routing;
