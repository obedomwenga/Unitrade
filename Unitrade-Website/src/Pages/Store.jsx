import React from "react";
import Nav from "../components/Nav";
import Help from "../components/Help";
import Footer from "../components/Footer";
import Products from "../components/Products";

function Store() {
    return (
        <div>
            <Nav />
            <Products />
            <Help />
            <Footer />
        </div>
    )
     
}

export default Store;