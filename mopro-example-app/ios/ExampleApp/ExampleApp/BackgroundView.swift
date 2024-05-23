//
//  BackgroundView.swift
//  ExampleApp
//
//  Created by 鄭雅文 on 2024/5/23.
//

import SwiftUI

struct BackgroundView: View {
    var body: some View {

            ZStack{
                Color(red: 37/255, green: 18/255, blue: 0/255).edgesIgnoringSafeArea(.all).frame(maxWidth: .infinity, maxHeight: .infinity)
                VStack{
                    TitleView()
                    BenchmarkView()
                }
            }
        
        
    }
}
