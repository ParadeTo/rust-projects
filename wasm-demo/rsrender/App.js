import React from 'react'

export const App = () => {
    return [...Array(10000)].map((_, i) => {
        return <div>Item {i + 1}</div>
    })
}
