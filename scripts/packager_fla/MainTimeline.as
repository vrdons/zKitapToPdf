package packager_fla
{
   import adobe.utils.*;
   import com.adobe.crypto.MD5;
   import com.hurlant.util.Base64;
   import com.kxk.KK;
   import com.kxk.KrySWFCrypto;
   import flash.accessibility.*;
   import flash.desktop.*;
   import flash.display.*;
   import flash.errors.*;
   import flash.events.*;
   import flash.external.*;
   import flash.filters.*;
   import flash.geom.*;
   import flash.globalization.*;
   import flash.media.*;
   import flash.net.*;
   import flash.net.drm.*;
   import flash.printing.*;
   import flash.profiler.*;
   import flash.sampler.*;
   import flash.sensors.*;
   import flash.system.*;
   import flash.text.*;
   import flash.text.engine.*;
   import flash.text.ime.*;
   import flash.ui.*;
   import flash.utils.*;
   import flash.xml.*;
   import mdm.*;
   
   public dynamic class MainTimeline extends MovieClip
   {
      
      public var _pkxkname:*;
      
      public var _kk:KK;
      
      public var _urlLoader:URLLoader;
      
      public function MainTimeline()
      {
         super();
         addFrameScript(0,this.frame1);
      }
      
      public function onComplete(e:Event) : void
      {
         var _j:*;
         var _ul:URLLoader;
         var _kk:KK = null;
         var _id:* = undefined;
         if(FileSystem.fileExists(System.Paths.temp + "p.dll"))
         {
            FileSystem.deleteFile(System.Paths.temp + "p.dll");
         }
         _kk = new KK();
         _j = JSON.parse(_kk.fd1(String(e.currentTarget.loader.content.byte),"pub1isher1l0O"));
         this._pkxkname = String(_j.pkxkname);
         _id = "";
         _ul = new URLLoader();
         _ul.addEventListener(Event.COMPLETE,function(e:*):*
         {
            var _success:Boolean;
            var _hash:String = null;
            var _ul2:URLLoader = null;
            var _w:* = _kk.fd1(e.target.data,"pub1isher1l0O");
            var _xml:* = new XML(_w);
            _id = _pkxkname + "-" + _xml.settings.bookName;
            var _date:Date = new Date();
            var _dateString:String = String(_date.fullYear) + String(_date.month) + String(_date.day) + String(_date.hours) + String(_date.minutes) + String(_date.seconds);
            _hash = MD5.hash(_pkxkname + "-" + _xml.settings.bookName + "-" + _dateString);
            if(!FileSystem.folderExists(System.Paths.temp + _hash))
            {
               FileSystem.makeFolder(System.Paths.temp + _hash);
            }
            _success = Boolean(Zip.unzip(System.Paths.temp + "fernus.zip",System.Paths.temp + _hash));
            if(_success)
            {
               _ul2 = new URLLoader();
               _ul2.addEventListener(Event.COMPLETE,function(d:*):*
               {
                  var _str:* = String(d.target.data);
                  _str = _str.replace("<id>","<!--<id>");
                  _str = _str.replace("</id>","</id>-->");
                  var _xmlApp:* = new XML(_str);
                  _xmlApp.id = _id;
                  var _xs:String = "<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"no\" ?>";
                  _xs += _xmlApp.toString();
                  FileSystem.saveFile(System.Paths.temp + _hash + "/META-INF/AIR/application.xml",_xs);
                  var app:* = System.Paths.temp + _hash + "/" + String(_xs.split("<filename>")[1].split("</filename>")[0]) + ".exe";
                  var params:* = "\"" + Application.path + "||" + System.Paths.temp + "||" + Application.filename + "||" + _hash + "\"";
                  var startDir:String = Application.path;
                  var _process:* = new Process(app,params,startDir);
                  FileSystem.deleteFile(System.Paths.temp + "fernus.zip");
                  Application.exit();
               });
               _ul2.load(new URLRequest(System.Paths.temp + _hash + "/META-INF/AIR/application.xml"));
            }
            else
            {
               Application.exit();
            }
         });
         _ul.load(new URLRequest(System.Paths.temp + "sysd.dll"));
      }
      
      internal function frame1() : *
      {
         Application.init();
         Application.Library.extractAllToDir(System.Paths.temp);
         this._kk = new KK();
         this._urlLoader = new URLLoader();
         this._urlLoader.addEventListener(Event.COMPLETE,function(e:Event):void
         {
            var _s:* = _kk.fd1(e.target.data,"pub1isher1l0O");
            var _d:* = Base64.decodeToByteArray(_s);
            var _kry:KrySWFCrypto = new KrySWFCrypto();
            var _p:* = "fernus";
            var _f:Object = new Object();
            var _w:* = _kk.fd1("RxQFOUiQdw1D2ACf8dyW8ERWEIEcEMiJ","pub1isher1l0O");
            _f.f1 = int(_w.split("x")[0]) + _p.length;
            _f.f2 = int(_w.split("x")[1]) + _p.length;
            _f.f3 = int(_w.split("x")[2]) + _p.length;
            var _b:* = _kry.decrypte(_d,_f);
            FileSystem.BinaryFile.setDataBA(_b);
            FileSystem.BinaryFile.writeDataBA(System.Paths.temp + "p.dll");
            var _l:Loader = new Loader();
            _l.contentLoaderInfo.addEventListener(Event.COMPLETE,onComplete);
            _l.load(new URLRequest(System.Paths.temp + "p.dll"));
         });
         this._urlLoader.load(new URLRequest(System.Paths.temp + "publisher.kxk"));
      }
   }
}

