package vaf_max_fla
{
   import flash.display.MovieClip;
   
   public dynamic class MainTimeline extends MovieClip
   {
      
      public var pkxkname:String;
      
      public var publisher:String;
      
      public var fernusCode:*;
      
      public var color:uint;
      
      public var server:String;
      
      public var passwordStatus:Boolean;
      
      public var operatingSystem:String;
      
      public var byte:*;
      
      public function MainTimeline()
      {
         super();
         addFrameScript(0,this.frame1);
      }
      
      internal function frame1() : *
      {
         this.pkxkname = "fernus";
         this.publisher = "FERNUS";
         this.fernusCode = "";
         this.color = 2301728;
         this.server = "http://www.fernus.com.tr/data_json.php";
         this.passwordStatus = false;
         this.operatingSystem = "windows";
         this.byte = "==ADfRzHFoXN0hHBpwxEHBDH9cieJkkGEciAloBAwI0Flp1A1QQEksgfYcgCsEwO85hEOUBLwRANXY3c0c2MGQyHrkQTRAUeH81fAkAJFJUSQZCO9dAH7whB5YAA+9wRn83B1QWO84FJaN1GNsjG0tlU8EhELsCVYwjJCo3XeATUQtjJ0JBIZkyWVx3OSMEAkQEI7cEfnRVVZQgLCZyWDMEJAYkZ5ZABDAxXaESQysyBPgwKLsxCGcwJFxDLSBiBFoyXC9Ff74VPhIVVEZAUloRMTRyFaYCWVMlIOFhHvw3D+sxF9lCQ1wVMjERVGcTOlozJfdEHQBBG9FwPZogYZl2GDk1G6cAF7YkFlViXD5gHhERVUA0AAEFF1ZgD7czILxSToQyBEYFJSohJrklKTETADtlHfElJQUgeGU";
      }
   }
}

