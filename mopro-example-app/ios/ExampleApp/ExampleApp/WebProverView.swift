//
//  WebProverView.swift
//  ExampleApp
//
//  Created by 鄭雅文 on 2024/5/23.
//

import SwiftUI
import WebKit

struct WebProverView: UIViewRepresentable{
    
    var url:String
    
    
    func makeUIView(context: Context) -> some UIView {
        guard let url = URL(string: url) else {
            return WKWebView()
        }
        let webview = WKWebView()
        webview.load(URLRequest(url: url))
        return webview
    }
    
    func updateUIView(_ uiView: UIViewType, context: Context) {
        
    }
}


#Preview {
    WebProverView(url: "https://web-prover.zkmopro.org")
}
