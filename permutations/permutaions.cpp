#include<bits/stdc++.h>
using namespace std;
 
int main() {
    ios_base::sync_with_stdio(false); cin.tie(NULL);
    int n,left,right;
    cin>>n;
    int mid=n/2;
    if(n==2){
       cout<<"NO SOLUTION"; 
    }
    else if(n==3) 
       cout<<"NO SOLUTION"; 
    else if(n==4) {
            cout<<"2 "<<"4 "<<"1 "<<"3";
       }
    else if(n%2==0 ){
    for (int i = 1; i <= mid; i++)
    {
        left=mid-i;
        right=mid+i;
        if(left==0) left=mid;
        cout<<left<<" "<<right<<" ";  
    }} else{
        mid+=1;
        for (int i = 1; i < mid; i++)
        {
            left=mid-i;
            right=mid+i;
            cout<<left<<" "<<right<<" ";
        }
       cout<<mid; 
    }
    
}